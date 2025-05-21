mod services;

use std::process;
use crate::services::change_env::change_env;
use crate::services::get_public_ip::get_public_ip;
use crate::services::open_android::open_android_studio;
use crate::services::open_wsl::open_wsl;
use crate::services::wsl_ip::wsl_ip;

fn main() {
    let wsl_ip_address = "172.18.51.114";
    let first_project_env = "D:\\Users\\Verdi\\AppData\\Local\\Android\\coffee_shop\\.env";
    let second_project_env = "D:\\Document\\Github\\go-coffee-shop\\.env";

    if let Err(e) = wsl_ip(wsl_ip_address) {
        eprintln!("Error checking WSL IP: {}", e);
        process::exit(1);
    }
    println!("Successfully check WSL IP!");


    let public_ip = match get_public_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Error getting public IP: {}", e);
            process::exit(1);
        }
    };
    println!("Successfully got IP: {}", public_ip);

    if let Err(e) = change_env(first_project_env, second_project_env, public_ip) {
        eprintln!("Error updating env: {}", e);
        process::exit(1);
    }
    println!("Successfully updating env!");

    if let Err(e) = open_android_studio() {
        eprintln!("Error opening Android Studio: {}", e);
        process::exit(1);
    }

    if let Err(e) = open_wsl() {
        eprintln!("Error opening WSL: {}", e);
        process::exit(1);
    }
}