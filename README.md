<div align="center">
  <h1>ReportBook</h1>
  <p></p>Simplistic automated diagnostic tool for Windows and macOS</p>
  
  ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Loudbooks/ReportBook/build.yml?style=for-the-badge)
</div>

ReportBook is a process used to diagnose a variety of possible issues on your device. You can find an example [here](https://pastebook.dev/pastes/millie-shadow-bard-milkchocolate?inspect).

This program was designed with the intention of being used by software support teams, but is available to anyone who wishes to use it. Source code can be viewed on [GitHub](https://github.com/Loudbooks/ReportBook) should you be interested in how it works.

All Windows releases have submitted to Microsoft for review, further ensuring your security.

### Collected Information
- Running Processes
- Installed Apps
- Hosts File
- Hardware Information (OS, CPU, GPU, RAM)

Your system username will be exempt from output.


# Usage
### Windows
1. Download the latest release [here](https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook.exe).
2. Double-click the downloaded executable.
3. Type in your preferred username and select `Continue`.
4. When the program has finished, select `Upload to PasteBook` at the bottom.
5. Copy this link and give it to whoever requested the log.

### macOS
1. Press `⌘ + Space`, type in `Terminal`, and press Enter. Copy and paste the following command into the terminal and press Enter. 
```bash
curl -O https://raw.githubusercontent.com/Loudbooks/ReportBook/master/macos-run.sh ; sh ./macos-run.sh
```
2. Type in your preferred username and select `Continue`.
3. When the program has finished, select `Upload to PasteBook` at the bottom.
4. Copy this link and give it to whoever requested the log.
