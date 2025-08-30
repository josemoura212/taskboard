fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("material".into());
    slint_build::compile_with_config("ui/app.slint", config).unwrap();

    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=build/icon.ico");
        println!("cargo:rerun-if-changed=build/app.rc");
        embed_resource::compile("build/app.rc", embed_resource::NONE);
    }
}
