#![windows_subsystem = "windows"]
slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    let weak_app = app.as_weak();
    slint::invoke_from_event_loop(move || weak_app.unwrap().window().set_maximized(true)).unwrap();

    app.on_quit(move || {
        std::process::exit(0);
    });

    app.run().map_err(|e| e.into())
}
