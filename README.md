# Youtube Downloader

A Youtube video downloader for Windows. Uses yt-dlp and deno.

# Download

Go to [this link](https://github.com/Kimi0n/youtube-downloader/releases) and download and extract the ZIP file. Run the exe.

# How to build (devs only)

1. Pull the repository.
2. Ensure Rust and npm are installed on your device.
3. Download and add [yt-dlp](https://github.com/yt-dlp/yt-dlp) and [deno](https://github.com/denoland/deno) to `src-tauri/bin` and rename it to:

`yt-dlp-x86_64-pc-windows-msvc.exe` `deno-x86_64-pc-windows-msvc.exe`

## Start dev server

```sh
npm run tauri dev
```

## Build the thing

```sh
npm run tauri build
```

# Features i want to implement for v1.0

- Add ffmpeg for 4k (this program is gonna be so huge)
- Improve progress indicator
- Allow user to specify a timestamp
- Quality selector
- Download audio only
- Allow user to select a folder to save to
- Abort button
- Youtube cookies import for locked content