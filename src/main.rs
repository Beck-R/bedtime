#![windows_subsystem = "windows"]

extern crate chrono;
extern crate rand;
extern crate system_shutdown;
extern crate winreg;

use chrono::prelude::*;
use std::{env, fs, thread, time};
use system_shutdown::shutdown;
use winreg::enums::*;
use winreg::RegKey;

fn main() {
    let mut path = fs::canonicalize(
        "C:\\Users\\".to_string()
            + &env::var("USERNAME").unwrap()
            + "\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\",
    )
    .unwrap();
    path.push("bedtime.exe");

    match fs::copy(env::current_exe().unwrap(), &path) {
        Ok(_) => println!("Copied to {}", path.to_str().unwrap()),
        Err(e) => println!("Error: {}", e),
    }

    // more covert(to the user) way of running program on startup, but is flagged by windows defender
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let key = hklm
        .open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
            KEY_WRITE,
        )
        .unwrap();
    key.set_value("Windows Auto Shutdown", &path.to_str().unwrap())
        .unwrap();

    loop {
        let now: DateTime<Local> = Local::now();
        let hour = now.hour();

        if hour >= 22 {
            println!("shutting down");
            match shutdown() {
                Ok(_) => println!("Shutting down, bye!"),
                Err(error) => eprintln!("Failed to shut down: {}", error),
            }
        } else if hour >= 1 && hour <= 6 {
            println!("shutting down");
            match shutdown() {
                Ok(_) => println!("Shutting down, bye!"),
                Err(error) => eprintln!("Failed to shut down: {}", error),
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
