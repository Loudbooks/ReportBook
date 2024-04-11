cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu

mkdir target/out

cp target/x86_64-apple-darwin/release/fullreport target/out/fullreport-macos-x86_64
cp target/aarch64-apple-darwin/release/fullreport target/out/fullreport-macos-aarch64
cp target/x86_64-unknown-linux-gnu/release/fullreport target/out/fullreport-linux-x86_64
cp target/aarch64-unknown-linux-gnu/release/fullreport target/out/fullreport-linux-aarch64
cp target/x86_64-pc-windows-gnu/release/fullreport.exe target/out/fullreport-windows-x86_64.exe
