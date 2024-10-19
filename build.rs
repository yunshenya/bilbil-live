fn main() {
    #[cfg(windows)]
    setup_windows_icon();

    #[cfg(unix)]
    setup_unix_icon();
}

#[cfg(windows)]
fn setup_windows_icon() {
    use winres::WindowsResource;
    WindowsResource::new()
        .set_icon("assets/bilibili.ico")
        .compile()
        .unwrap();
}

#[cfg(unix)]
fn setup_unix_icon() {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let out_dir = env::var("OUT_DIR").unwrap();
    let desktop_file = Path::new(&out_dir).join("bilibili.desktop");

    let mut file = File::create(&desktop_file).unwrap();
    write!(
        file,
        "[Desktop Entry]\n\
        Type=Application\n\
        Name=Bilibili\n\
        Exec=/usr/bin/bilibili\n\
        Icon=bilibili\n\
        Comment=Bilibili application\n\
        Categories=Utility;\n"
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Remember to install the icon file to /usr/share/icons/hicolor/256x256/apps/bilibili.png");
    println!("cargo:warning=and the .desktop file to /usr/share/applications/bilibili.desktop");
}
