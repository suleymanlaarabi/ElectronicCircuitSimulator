git add .
git commit -m "release"
git push -u origin main
cargo build --release
cargo build --target x86_64-pc-windows-gnu --release
mv ./target/release/circuit-sim ./target/release/circuit-sim-LINUX
mv ./target/x86_64-pc-windows-gnu/release/circuit-sim ./target/x86_64-pc-windows-gnu/release/circuit-sim-WINDOWS