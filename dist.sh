#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

git add -A
git commit -m. || true
git pull
git push
cargo publish --registry crates-io -p tokio-postgres
