use once_cell::sync::Lazy;
use std::sync::{Mutex, RwLock};
use std::time::Duration;
use std::thread;

pub static GLOBAL_ARGS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| {
    RwLock::new(vec![])
});

pub static ARGS_INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(false)
});

/// Set global arguments
pub fn set_args(args: &Vec<String>) {
    println!("Setting global args: {:?}", args);

    match GLOBAL_ARGS.write() {
        Ok(mut global) => {
            global.clear();
            global.extend(args.clone());

            if let Ok(mut initialized) = ARGS_INITIALIZED.lock() {
                *initialized = true;
            }

            println!("Global args successfully set");
        },
        Err(e) => {
            eprintln!("Failed to acquire write lock on GLOBAL_ARGS: {:?}", e);
        }
    }
}

/// Get all arguments as Vec<String>
pub fn get_args() -> Vec<String> {
    let mut attempts = 0;
    while attempts < 5 {
        if let Ok(initialized) = ARGS_INITIALIZED.lock() {
            if *initialized {
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
        attempts += 1;
    }

    match GLOBAL_ARGS.read() {
        Ok(vec) => vec.clone(),
        Err(e) => {
            eprintln!("Failed to acquire read lock on GLOBAL_ARGS: {:?}", e);
            vec![]
        }
    }
}

/// Get argument by index
pub fn get_arg(index: usize) -> Option<String> {
    let mut attempts = 0;
    while attempts < 5 {
        if let Ok(initialized) = ARGS_INITIALIZED.lock() {
            if *initialized {
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
        attempts += 1;
    }

    match GLOBAL_ARGS.read() {
        Ok(vec) => vec.get(index).cloned(),
        Err(e) => {
            eprintln!("Failed to acquire read lock on GLOBAL_ARGS for index {}: {:?}", index, e);
            None
        }
    }
}
