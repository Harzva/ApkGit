#[cfg(target_os = "windows")]
fn main() {
    if let Err(err) = winresource::WindowsResource::new()
        .set_icon("assets/brand/gitmarket-logo.ico")
        .compile()
    {
        println!("cargo:warning=failed to embed Windows icon: {err}");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
