cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu

mkdir target/out

cp target/x86_64-apple-darwin/release/reportbook target/out/reportbook-macos-x86_64
cp target/aarch64-apple-darwin/release/reportbook target/out/reportbook-macos-aarch64
cp target/x86_64-pc-windows-gnu/release/reportbook.exe target/out/reportbook-windows-x86_64.exe

upx --best --lzma target/out/reportbook-macos-x86_64
upx --best --lzma target/out/reportbook-macos-aarch64
upx --best --lzma target/out/reportbook-windows-x86_64.exe