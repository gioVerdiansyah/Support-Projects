use std::io;
use std::io::ErrorKind;
use Flutter::utils::create_folder::create_folder;
use Flutter::utils::global_args::get_args;
use crate::templates::contents::create_api::create_api;
use crate::templates::contents::create_cubit::create_cubit;
use crate::templates::contents::create_entity::create_entity;
use crate::templates::contents::create_model::create_model;
use crate::templates::contents::create_repository::create_repository;
use crate::templates::contents::create_repository_impl::create_repository_impl;
use crate::templates::contents::create_screen::create_screen;
use crate::templates::contents::create_state::create_state;

pub fn create_contents(only_modules: Option<Vec<&str>>) -> io::Result<()> {
    println!("=== Masuk ke create_contents ===");

    let all_modules = vec![
        "entity",
        "repository",
        "model",
        "api",
        "cubit",
        "repository_impl",
        "screen",
        "state",
    ];

    fn call_create(module: &str) -> io::Result<()> {
        match module {
            "entity" => create_entity(),
            "repository" => create_repository(),
            "model" => create_model(),
            "api" => create_api(),
            "cubit" => create_cubit(),
            "repository_impl" => create_repository_impl(),
            "screen" => create_screen(),
            "state" => create_state(),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Unknown module '{}'", module),
            )),
        }
    }

    if let Some(modules) = only_modules {
        for m in modules {
            println!("--> Memanggil module: {}", m);
            call_create(m)?;
        }
    } else {
        for m in all_modules.iter() {
            call_create(m)?;

            let args = get_args();
            let feature_name = args.get(0).cloned().unwrap_or_default();
            let folder_path = format!("lib\\src\\features\\{}\\presentations\\widgets", feature_name.to_lowercase());
            match create_folder(&folder_path) {
                Ok(_) => println!("Created folder: \"{}\"", &folder_path),
                Err(e) => eprintln!("Error while creating folder \"{}\"\n{}", folder_path, e)
            };
        }
    }
    Ok(())
}
