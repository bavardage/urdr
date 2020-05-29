use chrono::Utc;
use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use core_graphics::display::*;
use std::ffi::{c_void, CStr};
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};

unsafe fn get_window_layer(window_info: CFDictionaryRef) -> Option<i32> {
    let window_info = CFDictionary::<CFString, CFNumber>::wrap_under_get_rule(window_info);
    let window_layer_key = CFString::new("kCGWindowLayer");

    if window_info.contains_key(&window_layer_key) {
        window_info.get(window_layer_key).to_i32()
    } else {
        None
    }
}

unsafe fn get_window_name(window_info: CFDictionaryRef) -> Option<String> {
    let window_info = CFDictionary::<CFString, CFString>::wrap_under_get_rule(window_info);
    let window_owner_name_key = CFString::new("kCGWindowOwnerName");

    if window_info.contains_key(&window_owner_name_key) {
        Some(window_info.get(window_owner_name_key).to_string())
    } else {
        None
    }
}

unsafe fn get_active_window_title() -> Option<String> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;

    let window_list_info = CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID) as CFArrayRef;
    let window_list_info = CFArray::<CFDictionary>::wrap_under_create_rule(window_list_info);

    for item in window_list_info.into_iter() {
        let dic_ref = item.as_concrete_TypeRef();

        let window_layer = get_window_layer(dic_ref);
        if window_layer.is_none() {
            continue;
        }
        if window_layer.unwrap() != 0 {
            continue;
        }

        if let Some(name) = get_window_name(dic_ref) {
            return Some(name);
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let log_file_path = Utc::now().date().format("%Y-%m-%d.log.tsv");
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path.to_string())?;
    println!("Writing log to {}", log_file_path);

    loop {
        if let Some(active_window) = unsafe { get_active_window_title() } {
            let now = Utc::now().to_rfc3339();

            println!("{}\t{}\n", now, active_window);

            log_file.write_fmt(format_args!("{}\t{}\n", now, active_window))?;
            log_file.flush()?;
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}
