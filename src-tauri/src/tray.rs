use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Window, WindowEvent
};

pub const TRAY_LABEL: &'static str = "main-tray";

pub fn window_event_handler<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    label: &str,
    event: &WindowEvent,
) {
    match event {
        // Prevent Cinny from closing, instead hide it and let it be
        // reopened through the tray.
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();

            tauri::AppHandle::hide(
                &app.get_window(label)
                .unwrap().app_handle()
            ).unwrap();
        }
        _ => {}
    }
}

/// Build the system tray object
pub fn system_tray() -> SystemTray {
    let toggle = CustomMenuItem::new("show".to_owned(), "Show Cinny").accelerator("Command+S");
    let quit = CustomMenuItem::new("quit".to_owned(), "Quit").accelerator("Command+Q");
    let menu = SystemTrayMenu::new()
        .add_item(toggle)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::SystemTray::new()
        .with_menu(menu)
        .with_id(TRAY_LABEL.to_owned())
}

fn show_window<R: tauri::Runtime>(window: &Window<R>) {
    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn system_tray_handler<R: tauri::Runtime>(app: &tauri::AppHandle<R>, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();

    match event {
        SystemTrayEvent::LeftClick { .. } => {
            show_window(&window)
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    app.exit(0);
                }
                "show" => {
                    show_window(&window);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
