#[cfg(windows)]
fn main() {
    use winres::WindowsResource;
    WindowsResource::new()
        .set_icon("bilibili.ico")
        .compile()
        .unwrap();
}

#[cfg(unix)]
fn main() {}
