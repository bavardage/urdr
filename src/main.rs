use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use core_graphics::display::*;
use std::ffi::{c_void, CStr};
use std::{thread, time};

unsafe fn get_window_layer(window_info: CFDictionaryRef) -> Option<i32> {
    let window_layer_key = CFString::new("kCGWindowLayer");

    let mut value: *const c_void = std::ptr::null();
    if CFDictionaryGetValueIfPresent(window_info, window_layer_key.to_void(), &mut value) == 0 {
        return None;
    }

    let mut window_layer: i32 = 0;
    if !CFNumberGetValue(
        value as CFNumberRef,
        kCFNumberSInt32Type,
        &mut window_layer as *mut i32 as *mut c_void,
    ) {
        return None;
    }

    return Some(window_layer);
}

unsafe fn get_window_name(window_info: CFDictionaryRef) -> Option<String> {
    let window_owner_name_key = CFString::new("kCGWindowOwnerName");

    let mut value: *const c_void = std::ptr::null();
    if CFDictionaryGetValueIfPresent(window_info, window_owner_name_key.to_void(), &mut value) == 0
    {
        return None;
    }

    let c_ptr = CFStringGetCStringPtr(value as CFStringRef, kCFStringEncodingUTF8);
    if c_ptr.is_null() {
        return None;
    }

    let c_result = CStr::from_ptr(c_ptr);
    Some(String::from(c_result.to_str().unwrap()))
}

unsafe fn get_active_window_title() -> Option<String> {
    const OPTIONS: CGWindowListOption =
        kCGWindowListOptionOnScreenOnly | kCGWindowListExcludeDesktopElements;

    let window_list_info = CGWindowListCopyWindowInfo(OPTIONS, kCGNullWindowID);
    let count = CFArrayGetCount(window_list_info);

    for i in 0..count {
        let dic_ref = CFArrayGetValueAtIndex(window_list_info, i as isize) as CFDictionaryRef;

        let window_layer = get_window_layer(dic_ref);
        if window_layer.is_none() {
            continue;
        }
        if window_layer.unwrap() != 0 {
            continue;
        }

        if let Some(name) = get_window_name(dic_ref) {
            CFRelease(window_list_info as CFTypeRef);
            return Some(name);
        }
    }

    CFRelease(window_list_info as CFTypeRef);
    None
}

fn main() {
    loop {
        let active_window = unsafe { get_active_window_title() };
        println!("Active: {}", active_window.unwrap());
        thread::sleep(time::Duration::from_millis(1000));
    }
}
