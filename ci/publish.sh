#!/bin/bash

crate=$1

case ${crate} in
    dose-derive)
        cd dose-derive/
        echo "Publishing dose derive ..."
        ;;
    dose)
        echo "Publishing dose ..."
        ;;
    *)
        echo "Crate ${crate} unknown"
        exit 1
        ;;
esac

cargo publish --token ${CRATES_IO_API_TOKEN} || exit 1
echo "Sucessfully published ${crate}"
