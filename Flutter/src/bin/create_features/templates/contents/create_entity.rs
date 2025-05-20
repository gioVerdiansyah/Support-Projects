use std::io;
use std::path::PathBuf;
use Flutter::utils::create_file::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_entity() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();

    let content = &format!(r#"
class {0}Entity {{

  {0}Entity(){{
  }}

  factory {0}Entity.empty() {{
    return {0}Entity();
  }}
}}"#, capitalize_first_letter(&*feature_name));

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\domain\\entities", feature_name.to_lowercase()))
        .join(&format!("{}_entity.dart", feature_name.to_lowercase()));

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match create_file_content(&file_path, content.clone()) {
        Ok(_) => println!("Created file: '{:?}'", file_path),
        Err(e) => eprintln!("Error creating file '{:?}' \n {:?}", file_path, e),
    };

    Ok(())
}