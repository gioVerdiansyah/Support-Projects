mod templates;

use Flutter::utils::global_args::set_args;
use crate::templates::create_contents::create_contents;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Error: must be add feature name and --package_name");
        std::process::exit(1);
    }

    let feature_name = &args[0];

    let package_name = args.iter()
        .find(|arg| arg.starts_with("--package_name="))
        .map(|s| s.trim_start_matches("--package_name=").to_string());

    if package_name.is_none() {
        eprintln!("Error: --package_name must be added!");
        std::process::exit(1);
    }

    let only_param = args.iter()
        .find(|arg| arg.starts_with("--only="))
        .map(|s| s.trim_start_matches("--only=").to_string());

    // Parsing --only
    let binding = only_param.clone();
    let only_modules: Option<Vec<&str>> = binding
        .as_ref()
        .map(|s| s.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).collect());

    let name = args.iter()
        .find(|arg| arg.starts_with("--name="))
        .map(|s| s.trim_start_matches("--name=").to_string());

    if only_param.is_some() ^ name.is_some() {
        eprintln!("Error: '--only=' and '--name=' must be used together!");
        std::process::exit(1);
    }

    let name = name.unwrap_or_default();

    let cleaned_args = if name.is_empty() {
        vec![
            feature_name.to_string(),
            package_name.clone().unwrap(),
        ]
    } else {
        vec![
            feature_name.to_string(),
            package_name.clone().unwrap(),
            name
        ]
    };

    set_args(&cleaned_args);

    match create_contents(only_modules){
        Ok(_) => println!("Successfully create features!"),
        Err(e) => eprintln!("Failed to create features!\n{}", e)
    };
}
