use std::io;
use std::path::PathBuf;
use Flutter::utils::files::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_api() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();
    let name = args.get(2).cloned().unwrap_or_else(|| feature_name.clone());

    let content = &format!(r#"
import 'package:{1}/src/core/exceptions/api_exception.dart';
import 'package:{1}/src/core/network/api_client.dart';
import 'package:{1}/src/core/network/api_path.dart';
import 'package:{1}/src/features/{2}/data/models/{2}_model.dart';

class {0}Api {{
  final ApiClient _api;

  {0}Api(this._api);

  Future<{0}Model> example() async {{
    final results = await _api.get(ApiPath.example);

    return {0}Model.fromJson(results.data);
  }}
}}"#, capitalize_first_letter(&*name), package_name, name.to_lowercase());

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\data\\api", feature_name.to_lowercase()))
        .join(&format!("{}_api.dart", name.to_lowercase()));

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