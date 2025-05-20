use std::io;
use std::path::PathBuf;
use Flutter::utils::create_file::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_model() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();

    let content = &format!(r#"
import 'package:{1}/src/features/{2}/domain/entities/{2}_entity.dart';

class {0}Model extends {0}Entity {{


  {0}Model({{

  }});

  factory {0}Model.fromJson(Map<String, dynamic> json) => {0}Model(

  );

  {0}Entity toEntity() {{
    return {0}Entity();
  }}
}}"#, capitalize_first_letter(&*feature_name), package_name, feature_name.to_lowercase());

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\data\\models", feature_name.to_lowercase()))
        .join(&format!("{}_model.dart", feature_name.to_lowercase()));

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