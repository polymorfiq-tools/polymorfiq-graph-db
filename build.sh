#!/usr/bin/env bash
set -euxo pipefail;

# Generate WASM file
ORIG_WASM_FILE="wasm/target/wasm32-unknown-unknown/release/wasm.wasm";
WASM_FILE="wasm/target/wasm32-unknown-unknown/release/wasm-optimized.wasm";
pushd wasm && cargo build --release $@ && popd;
wasm-opt $ORIG_WASM_FILE -o $WASM_FILE --strip-debug -Oz;

# Copy to relevant language library ports
ORIG_WASM_SIZE=$(stat -f%z $ORIG_WASM_FILE);
ORIG_WASM_SIZE_PRETTY=$(numfmt --to=iec-i --suffix=B --format="%9.2f" $ORIG_WASM_SIZE | tr -d '[:space:]');
WASM_SIZE=$(stat -f%z $WASM_FILE);
WASM_SIZE_PRETTY=$(numfmt --to=iec-i --suffix=B --format="%9.2f" $WASM_SIZE | tr -d '[:space:]');

cp $WASM_FILE languages/elixir/priv/wasm.wasm;
cp $WASM_FILE languages/nodejs/src/vendor/wasm.wasm;
cp $WASM_FILE languages/react-app/public/vendor/wasm.wasm;

# If wat-wasm is on the system, let's generate a text-based WASM for review
if command -v wat-wasm &> /dev/null
then
    wat-wasm $WASM_FILE -o bin/latest-optimized.wat;
    wat-wasm $ORIG_WASM_FILE -o bin/latest.wat;
fi

printf "\nThe WASM filesize is $ORIG_WASM_SIZE_PRETTY -> $WASM_SIZE_PRETTY\n\n";