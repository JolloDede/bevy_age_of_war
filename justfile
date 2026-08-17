
run:
    RUST_LOG=error,bevy_age_of_war=debug cargo run

bundle:
    cargo build --release
    zip -r release.zip assets target/release/bevy_age_of_war
