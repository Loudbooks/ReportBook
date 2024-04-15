mkdir ./temp

if [[ $(uname -m) == 'arm64' ]]; then
    cd ./temp && curl -LO "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-aarch64"
else 
    cd ./temp && curl -LO "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-x86_64"
fi

chmod +x ./reportbook-macos-aarch64
./reportbook-macos-aarch64

rm -r ./temp
