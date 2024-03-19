use tauri::{AboutMetadata, Menu, MenuItem, Submenu};
//for macOS
pub(crate) fn menu() -> Menu {
    Menu::new()
        .add_submenu(Submenu::new(
            "Cinny",
            Menu::new()
                .add_native_item(MenuItem::About(
                    "Cinny".to_string(),
                    AboutMetadata::new()
                        .authors(vec![
                            "Cinny Development Team".to_string(),
                            "Iewnfod".to_string()
                        ])
                        .website("https://github.com/iewnfod/cinny-desktop")
                        .website_label("Github Repository")
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_native_item(MenuItem::Undo)
                .add_native_item(MenuItem::Redo)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Cut)
                .add_native_item(MenuItem::Copy)
                .add_native_item(MenuItem::Paste)
                .add_native_item(MenuItem::SelectAll),
        ))
        .add_submenu(Submenu::new(
            "View",
            Menu::new()
                .add_native_item(MenuItem::EnterFullScreen),
        ))
        .add_submenu(Submenu::new(
            "Window",
            Menu::new()
                .add_native_item(MenuItem::CloseWindow)
                .add_native_item(MenuItem::Minimize)
                .add_native_item(MenuItem::Zoom),
        ))
}
