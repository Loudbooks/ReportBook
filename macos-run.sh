if [[ $(uname -m) == 'arm64' ]]; then
    cd /tmp && curl -LO "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-aarch64"
    chmod +x ./reportbook-macos-aarch64
    ./reportbook-macos-aarch64
    rm ./reportbook-macos-aarch64
else
    cd /tmp && curl -LO "https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-macos-x86_64"
    chmod +x ./reportbook-macos-x86_64
    ./reportbook-macos-x86_64
    rm ./reportbook-macos-x86_64
fi
