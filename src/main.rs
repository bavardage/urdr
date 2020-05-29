use chrono::Utc;
use core_foundation::base::*;
use core_foundation::number::*;
use core_foundation::string::*;
use core_graphics::display::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{thread, time};
use structopt::StructOpt;

mod chrome;

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

    let window_list_info: CFArray = CGDisplay::window_list_info(OPTIONS, None).unwrap();

    for item in window_list_info.into_iter() {
        let dic_ref = *item as CFDictionaryRef;

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

#[derive(Serialize)]
struct Record {
    timestamp: String,
    window_title: String,
    current_url: Option<String>,
}

#[derive(StructOpt)]
struct Cli {
    /// The directory to output log files. Defaults to current directory.
    #[structopt(parse(from_os_str))]
    path: Option<PathBuf>,
}

fn main() -> Result<(), ExitFailure> {
    let args: Cli = Cli::from_args();

    let output_directory = args.path.unwrap_or(PathBuf::from("."));
    if !output_directory.is_dir() {
        return Err(std::io::Error::from(ErrorKind::InvalidInput)).with_context(|_| {
            format!(
                "Output directory was not a directory: {:?}",
                output_directory
            )
        })?;
    }

    let log_file_path = Utc::now().date().format("%Y-%m-%d.log.csv");
    let full_path = std::fs::canonicalize(output_directory)?.join(log_file_path.to_string());

    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&full_path)
        .with_context(|_| format!("Unable to open output file: {:?}", &full_path))?;

    let mut csv_writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(log_file);

    println!("Writing log to {}", &full_path.display());

    let bar = indicatif::ProgressBar::new_spinner();
    bar.enable_steady_tick(100);

    loop {
        if let Some(active_window) = unsafe { get_active_window_title() } {
            let now = Utc::now();

            let current_url = if active_window == "Google Chrome" {
                chrome::get_active_tab_url()
            } else {
                None
            };

            let record = Record {
                timestamp: now.to_rfc3339(),
                window_title: active_window.clone(),
                current_url: current_url,
            };

            bar.set_message(active_window.as_str());

            csv_writer.serialize(record)?;
            csv_writer.flush()?;
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}
