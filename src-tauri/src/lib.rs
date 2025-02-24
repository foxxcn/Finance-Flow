// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }));
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .with_webview(|webview| {
                    #[cfg(target_os = "linux")]
                    {
                        use webkit2gtk::WebViewExt;
                        webview.inner().set_zoom_level(1.0);
                    }

                    #[cfg(target_os = "windows")]
                    unsafe {
                        webview.controller().SetZoomFactor(1.0).unwrap();
                    }

                    #[cfg(target_os = "macos")]
                    unsafe {
                        let view: &objc2_web_kit::WKWebView = &*webview.inner().cast();
                        view.setPageZoom(1.0);
                    }
                })
                .unwrap();

            // turn on clearing browsing data, unsaved local storage for auto login
            let window = main_window.clone();
            main_window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    let _ = window.clear_all_browsing_data();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
