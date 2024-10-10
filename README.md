<div align="center">
  <div id="user-content-toc">
    <ul align="center" style="list-style: none;">
      <summary>
        <h1>ReportBook</h1>
      </summary>
    </ul>
  </div>
  <p></p>Simplistic automated diagnostic tool for Windows and macOS written in <a href="https://tauri.app/">Tauri</a> and <a href="https://www.rust-lang.org/">Rust</a></p>  
</div>

------------

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
1. Download the latest release [here](https://github.com/Loudbooks/ReportBook/releases/latest/download/reportbook-windows.exe).
2. Double-click the downloaded executable.
> [!IMPORTANT]
> **Smartscreen Warning**
>
> Yeah. I know. I cannot afford a signing certificate, as they cost upwards of $300 USD. This does not mean the code isn't secure, or that you can't trust it. It means I haven't paid the price.
>
> Select more info, and select run anyway.

4. Type in your preferred username and select `Continue`.
5. When the program has finished, select `Upload to PasteBook` at the bottom.
6. Copy this link and give it to whoever requested the log.

### macOS
1. Press `⌘ + Space`, type in `Terminal`, and press Enter. Copy and paste the following command into the terminal and press Enter. 
```bash
curl -O -s https://raw.githubusercontent.com/Loudbooks/ReportBook/master/macos-run.sh ; sh ./macos-run.sh ; rm ./macos-run.sh
```
2. Type in your preferred username and select `Continue`.
3. When the program has finished, select `Upload to PasteBook` at the bottom.
4. Copy this link and give it to whoever requested the log.
