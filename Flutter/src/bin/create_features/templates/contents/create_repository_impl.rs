use std::io;
use std::path::PathBuf;
use Flutter::utils::create_file::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_repository_impl() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();

    let content = &format!(r#"
import 'package:{1}/src/core/exceptions/api_exception.dart';
import 'package:{1}/src/core/exceptions/repository_exception.dart';
import 'package:{1}/src/features/{2}/domain/entities/{2}_entity.dart';
import 'package:{1}/src/features/{2}/domain/repositories/{2}_repository.dart';

class {0}RepositoryImpl implements {0}Repository {{
  final {0}Api _api;

  {0}RepositoryImpl(this._api);

  @override
  Future<{0}Entity> example() async {{
    try {{
      final res = await _api.example();

      return res.toEntity();
    }} on ApiException {{
      rethrow;
    }} catch (e, stackTrace) {{
      throw RepositoryException(
        e.toString(),
        stackTrace: stackTrace,
        source: "{0}RepositoryImpl",
        details: "...",
      );
    }}
  }}
}}"#, capitalize_first_letter(&*feature_name), package_name, feature_name.to_lowercase());

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\data\\repositories", feature_name.to_lowercase()))
        .join(&format!("{}_repository_impl.dart", feature_name.to_lowercase()));

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