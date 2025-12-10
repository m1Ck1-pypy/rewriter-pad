fn main() {
    let os = match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(os) => os,
        Err(_) => String::from("unknown"),
    };

    match os.as_str() {
        "windows" => {
            let mut res = winres::WindowsResource::new();

            res.set_icon("icon.ico") // Используйте .ico файл для Windows
                .set("ProductName", "Rewriter Pad")
                .set("FileDescription", "Rewriter Pad");
            res.compile().expect("Failed to compile windows resources");
        }
        "linux" => {}
        "macos" => {}
        _ => {}
    }
}
