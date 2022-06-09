#!/bin/bash

echo "Publishing dose-derive ..."
cd dose-derive/
cargo publish --token ${CRATES_IO_API_TOKEN}
echo "Sucessfully published dose-derive"

echo "Waiting 10 seconds for dose-derive to be available ..."
sleep 10

echo "Publishing dose ..."
cd ../
cargo publish --token ${CRATES_IO_API_TOKEN}
echo "Sucessfully published dose"
