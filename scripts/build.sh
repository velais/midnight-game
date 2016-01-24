echo "Building Midnight"

cd $(dirname "$0")
cargo build --manifest-path='../midnight/Cargo.toml'
