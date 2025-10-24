pub async fn set_console_title() {
    #[cfg(target_os = "windows")]
    {
        use winconsole::console;
        let app_title = env!("CARGO_PKG_NAME");
        console::set_title(app_title).unwrap();
    }
}

