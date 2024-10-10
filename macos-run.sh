if [[ $(uname -m) == 'arm64' ]]; then
    echo Downloading the aarch64 version of ReportBook...
    cd /tmp && curl -LO -s "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-aarch64"
    chmod +x ./reportbook-macos-aarch64
    ./reportbook-macos-aarch64
    rm ./reportbook-macos-aarch64
else
    echo Downloading the x86_64 version of ReportBook...
    cd /tmp && curl -LO -s "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-x86_64"
    chmod +x ./reportbook-macos-x86_64
    ./reportbook-macos-x86_64
    rm ./reportbook-macos-x86_64
fi
