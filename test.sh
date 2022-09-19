#!/usr/bin/env bash
set -euxo pipefail;

pushd lib;
cargo test
cargo test --features alloc
popd;

pushd languages/elixir;
mix deps.get;
mix test;
popd;

pushd languages/nodejs;
npm install;
npm test;
popd;

pushd languages/react-app;
npm install;
npm test -- --watchAll=false;
popd;