use std::io;
use std::path::PathBuf;
use Flutter::utils::create_file::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_state() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();

    let content = &format!(r#"
import 'package:{1}/src/features/{2}/domain/entities/{2}_entity.dart';
import 'package:{1}/src/shared/state/api_state.dart';
import 'package:equatable/equatable.dart';

class {0}State extends Equatable {{
  final ApiState api;
  final {0}Entity {2};

  {0}State({{
    ApiState? api,
    {0}Entity? user,
  }})  : api = api ?? ApiState(),
        {2} = {2} ?? {0}Entity.empty();

  {0}State copyWith({{
    ApiState? api,
    {0}Entity? {2},
  }}) {{
    return {0}State(
      api: api ?? this.api,
      {2}: {2} ?? this.{2},
    );
  }}

  @override
  List<Object?> get props => [api, {2}];
}}"#, capitalize_first_letter(&*feature_name), package_name, feature_name.to_lowercase());

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\presentations\\blocs\\state", feature_name.to_lowercase()))
        .join(&format!("{}_state.dart", feature_name.to_lowercase()));

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