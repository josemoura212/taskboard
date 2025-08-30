#![windows_subsystem = "windows"]
slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    let weak_app = app.as_weak();
    slint::invoke_from_event_loop(move || weak_app.unwrap().window().set_maximized(true)).unwrap();

    let app_weak = app.as_weak();
    app.on_toggle_theme(move || {
        let app = app_weak.unwrap();
        let current = app.get_current_theme();
        if current == "light" {
            app.set_current_theme("dark".into());
        } else {
            app.set_current_theme("light".into());
        }
    });

    app.run().map_err(|e| e.into())
}
