#![windows_subsystem = "windows"]
slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = AppWindow::new()?;

    app.on_quit(move || {
        std::process::exit(0);
    });

    app.run().map_err(|e| e.into())
}
