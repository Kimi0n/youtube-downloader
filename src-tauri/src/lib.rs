use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Emitter;

struct DownloadMetadata {
    download_percentage: String,
    file_size: String,
    download_speed: String, 
    eta: String
}

impl DownloadMetadata {
    fn is_unknown(&self) -> bool {
        let mut unknown = false;

        if self.download_percentage == "Unknown" {
            unknown = true;
        }

        if self.file_size == "Unknown" {
            unknown = true;
        }

        if self.download_speed == "Unknown" {
            unknown = true;
        }

        if self.eta == "Unknown" {
            unknown = true;
        }

        return unknown
    }
}

#[tauri::command]
async fn download_from_youtube(app: tauri::AppHandle, youtube_id: String) {
    call_ytdlp_for_download(app, &youtube_id).await;
}

fn extract_download_metadata(line: &str) -> DownloadMetadata {
    let parts: Vec<&str> = line.split_whitespace().collect();
    // println!("{:?}", parts);

    if parts.len() < 8 {
        return DownloadMetadata { download_percentage: "Unknown".to_string(), file_size: "Unknown".to_string(), download_speed: "Unknown".to_string(), eta: "Unknown".to_string() }
    }

    return DownloadMetadata { download_percentage: parts[1].to_string(), file_size: parts[3].to_string(), download_speed: parts[5].to_string(), eta: parts[7].to_string() }
}

async fn call_ytdlp_for_download(app: tauri::AppHandle, youtube_id: &str) {
    let ytdlp_args = vec![
        "--newline",
        "-o", "downloads/%(title)s.%(ext)s",
        "-t", "mp4",
        youtube_id
    ];

    let log_file_path = "output.log";
    let _write_result = fs::write(log_file_path, "Logs from: call_ytdlp_for_download\r\n");
    let _ = write_to_file(log_file_path, format!("yt-dlp args: {:?}\r\n", ytdlp_args).as_str());

    // println!("{:#?}", ytdlp_args);

    let ytdlp_command = app.shell()
        .sidecar("yt-dlp") 
        .unwrap()
        .args(ytdlp_args);

    let (mut rx, _child) = ytdlp_command
        .spawn()
        .expect("failed to spawn yt-dlp");

    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => { // Not used
                    let line = String::from_utf8_lossy(&line_bytes);
                    let _ = write_to_file(log_file_path, &line);

                    if line.contains("[download]") {
                        let current_metadata: DownloadMetadata = extract_download_metadata(&line);
                        // println!("{} {} {} {}", current_metadata.download_percentage, current_metadata.file_size, current_metadata.download_speed, current_metadata.eta);

                        if !current_metadata.is_unknown() {
                            app.emit("yt-dlp-progress", format!("Progress: {} - Size: {} - Speed: {} - ETA: {}", current_metadata.download_percentage, current_metadata.file_size, current_metadata.download_speed, current_metadata.eta)).unwrap();
                        }
                    }
                    
                    // println!("STDOUT:: {}", line);
                },
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    let _ = write_to_file(log_file_path, &line);
                    println!("STDERR:: {}", line);
                },
                CommandEvent::Terminated(payload) => {
                    app.emit("yt-dlp-finished", payload.code).unwrap();
                    //println!("{:#?}", payload.code);
                    break;
                },
                _ => {}
            }
        }
    });
}

fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init()) 
        .invoke_handler(tauri::generate_handler![download_from_youtube])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
