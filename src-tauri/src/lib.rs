// skeleton: proves the build, the plugins and the installers work on Windows and Mac
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running Work Timer");
}
