#!/bin/bash

cd dose-derive/
cargo publish --token ${CRATES_IO_API_TOKEN}
cd ../
cargo publish --token ${CRATES_IO_API_TOKEN}
