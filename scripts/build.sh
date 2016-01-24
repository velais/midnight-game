echo "Building Midnight"

cd $(dirname "$0")
cargo test --mainifest-path='../midnight/Cargo.toml'
cargo build --manifest-path='../midnight/Cargo.toml' --release
