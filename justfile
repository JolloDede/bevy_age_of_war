
run:
    RUST_LOG=error,bevy_age_of_war=debug cargo run

bundle:
    cargo build --release
    zip -r release.zip assets target/release/bevy_age_of_war

wasm:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web \
        --out-dir ./out/ \
        --out-name "ageofwar" \
        ./target/wasm32-unknown-unknown/release/bevy_age_of_war.wasm
