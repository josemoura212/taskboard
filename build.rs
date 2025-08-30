#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("material".into());
    slint_build::compile_with_config("ui/App.slint", config).unwrap();
}
