use std::env;

fn main() {
    let families: Vec<_> = env::vars().filter_map(|(key, _value)| {
        if key.starts_with("CARGO_FEATURE_STM32") && !key.ends_with("HAL") {
            Some(key[14..].to_ascii_lowercase())  // Strip 'CARGO_FEATURE_'
        } else {
            None
        }
    }).collect();

    if families.is_empty() {
        panic!("No family features selected");
    }
    if families.len() > 1 {
        panic!("More than one family feature selected: {:?}", families);
    }

    let family = families.first().unwrap();

    let buffer_size;
    let access_scheme;
    let lpm_support;
    let bcd_support;
    let dp_pull_up_support;
    match family.as_str() {
        "stm32f103xx" => {
            buffer_size = 512;
            access_scheme = "1x16";
            lpm_support = false;
            bcd_support = false;
            dp_pull_up_support = false;
        },
        "stm32l4x2xx" | "stm32f042xx" => {
            buffer_size = 1024;
            access_scheme = "2x16";
            lpm_support = true;
            bcd_support = true;
            dp_pull_up_support = true;
        },
        other => panic!("Unknown family: {}", other),
    }

    println!("cargo:rustc-cfg=usb_buffer_size=\"{}\"", buffer_size);
    println!("cargo:rustc-cfg=usb_access_scheme=\"{}\"", access_scheme);
    if lpm_support {
        println!("cargo:rustc-cfg=usb_lpm_support");
    }
    if bcd_support {
        println!("cargo:rustc-cfg=usb_bcd_support");
    }
    if dp_pull_up_support {
        println!("cargo:rustc-cfg=usb_dp_pull_up_support");
    }
}
