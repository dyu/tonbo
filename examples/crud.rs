use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use std::env;
use std::ops::Bound;

use futures_util::stream::StreamExt;
use tonbo::{executor::tokio::TokioExecutor, tonbo_record, Projection, DB};

fn is_truthy(str: String) -> bool {
    str == "1" || str == "true"
}

/// Use macro to define schema of column family just like ORM
/// It provides type-safe read & write API
#[tonbo_record]
pub struct User {
    #[primary_key]
    name: String,
    email: Option<String>,
    age: u8,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len() - 1;
    let mut i = 0;
    
    let db_path = format!("./db_path/{}", if len > i {
        &args[{ i += 1; i }]
    } else {
        "crud"
    });
    
    let name = if len > i {
        args[{ i += 1; i }].clone()
    } else {
        "Alice".into()
    };
    
    let age: u8 = if len > i {
        u8::from_str_radix(&args[{ i += 1; i }], 10).unwrap()
    } else {
        22
    };
    
    // pluggable async runtime and I/O
    let db = DB::new(db_path.into(), TokioExecutor::default())
        .await
        .unwrap();
    
    let insert = env::var("INSERT").is_ok_and(is_truthy);
    if insert {
        // insert with owned value
        db.insert(User {
            name: name.clone(),
            email: Some(format!("{}@gmail.com", name.to_lowercase())),
            age: age,
        })
        .await
        .unwrap();
    }
    {
        // tonbo supports transaction
        let txn = db.transaction().await;
        {
            // get the zero-copy reference of record without any allocations.
            let user = txn
                .get(
                    &name,
                    // tonbo supports pushing down projection
                    Projection::All,
                )
                .await
                .unwrap();
            if insert {
                assert!(user.is_some());
                assert_eq!(user.unwrap().get().name, name);
            } else if user.is_some() {
                let found = user.unwrap();
                let u = found.get();
                println!("Get: {} | {} | {}\n", u.name, u.email.unwrap(), u.age.unwrap());
            } else {
                println!("Not found: {name}\n");
            }
        }
        {
            let upper = "~".into();
            // range scan of user
            let mut scan = txn
                .scan((Bound::Included(&name), Bound::Excluded(&upper)))
                .await
                // tonbo supports pushing down projection
                .projection(vec![1])
                // push down limitation
                //.limit(1)
                .take()
                .await
                .unwrap();
            while let Some(entry) = scan.next().await.transpose().unwrap() {
                let u = entry.value().unwrap();
                println!("Scanned: {} | {} | {}", u.name, u.email.unwrap(), u.age.unwrap());
            }
        }

        // commit transaction
        txn.commit().await.unwrap();
    }
}
