fn main() {
    if cfg!(target_os = "windows") && std::env::var("CARGO_PKG_NAME").unwrap() == "DecryptTruck" {
        let mut res = winres::WindowsResource::new();
        res.set("FileDescription", env!("CARGO_PKG_DESCRIPTION"));
        res.set("ProductName", env!("CARGO_PKG_NAME"));
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
        res.compile().unwrap();
    }
}
