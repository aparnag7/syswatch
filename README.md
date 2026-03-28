A lightweight, live system monitor for your terminal.
Built with
1. ratatui — terminal UI framework
2. sysinfo — system stats
3. crossterm — terminal input and control

Make sure you have Rust installed.

`syswatch` runs in your terminal and shows a live-updating dashboard of your system stats:

1. CPU usage — real-time percentage with a progress bar
2. RAM usage — used vs total memory in MB
3. Disk usage — used vs total disk space in GB across all drives

Run `cargo run`
Everything refreshes every 500ms. Press `q` to exit cleanly.


<img width="1450" height="231" alt="image" src="https://github.com/user-attachments/assets/5aeb25fd-ba3f-4f16-96b6-058a68b35614" />

