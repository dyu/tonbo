#!/bin/sh

set -e

CURRENT_DIR=$PWD
# locate
if [ -z "$BASH_SOURCE" ]; then
    SCRIPT_DIR=`dirname "$(readlink -f $0)"`
elif [ -e '/bin/zsh' ]; then
    F=`/bin/zsh -c "print -lr -- $BASH_SOURCE(:A)"`
    SCRIPT_DIR=`dirname $F`
elif [ -e '/usr/bin/realpath' ]; then
    F=`/usr/bin/realpath $BASH_SOURCE`
    SCRIPT_DIR=`dirname $F`
else
    F=$BASH_SOURCE
    while [ -h "$F" ]; do F="$(readlink $F)"; done
    SCRIPT_DIR=`dirname $F`
fi

BASE_DIR=`dirname $SCRIPT_DIR`

cd $BASE_DIR

rm -rf db_path/guests

INSERT=1 ./scripts/crud.sh users Jake 5 > /dev/null
INSERT=1 ./scripts/crud.sh guests Jake 5 > /dev/null
INSERT=1 ./scripts/crud.sh guests John 10 > /dev/null
INSERT=1 ./scripts/crud.sh guests George 15 > /dev/null
INSERT=1 ./scripts/crud.sh guests Adam 20 > /dev/null
INSERT=1 ./scripts/crud.sh guests Adam 17 > /dev/null
INSERT=1 ./scripts/crud.sh guests Larry 25 > /dev/null

printf '# ======================\n# Expecting Not found: Luke\n'
./scripts/crud.sh guests Luke
printf '# ======================\n# Expecting Not found: Barry\n'
./scripts/crud.sh guests Barry
printf '# ======================\n# Expecting Get: Adam\n'
./scripts/crud.sh guests Adam
