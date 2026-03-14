use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[tauri::command]
async fn download_from_youtube(app: tauri::AppHandle, youtube_id: String) {
    call_ytdlp_for_download(app, &youtube_id).await;
}

async fn call_ytdlp_for_download(app: tauri::AppHandle, youtube_id: &str) {
    let ytdlp_args = vec![
        "--newline",
        "-o", "downloads/%(title)s.%(ext)s",
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
                    println!("STDOUT:: {}", line);
                },
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    let _ = write_to_file(log_file_path, &line);
                    println!("STDERR:: {}", line);
                },
                CommandEvent::Terminated(payload) => {
                    println!("{:#?}", payload.code);
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
