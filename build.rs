fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    if profile == "debug" {
        println!("cargo:warning=stm32-usbd is being compiled in debug mode. This driver works reliably only in release mode!");
    }
}
