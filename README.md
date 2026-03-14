# Youtube Downloader

A Youtube video downloader for Windows. Uses yt-dlp.

# Download

Go to [this link](https://www.google.com/) and download and extract the ZIP file. Run the exe.

# How to build (devs only)

1. Pull the repository.
2. Ensure Rust and npm are installed on your device.
3. Download and add [yt-dlp](https://github.com/yt-dlp/yt-dlp) to `src-tauri/bin` and rename it to:

`yt-dlp-x86_64-pc-windows-msvc.exe`

## Start dev server

```sh
npm run tauri dev
```

## Build the thing

```sh
npm run tauri build
```

# Features i want to implement for v1.0

- Block download button when downloading
- Loading curcle with %, filesize and speed
- Allow user to specify a timestamp
- Download audio only
- Allow user to select a folder to save to