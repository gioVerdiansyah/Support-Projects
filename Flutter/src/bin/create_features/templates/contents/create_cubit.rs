use std::io;
use std::path::PathBuf;
use Flutter::utils::files::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_cubit() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();
    let name = args.get(2).cloned().unwrap_or_else(|| feature_name.clone());

    let content = &format!(r#"
import 'package:{1}/src/core/exceptions/api_exception.dart';
import 'package:{1}/src/core/exceptions/repository_exception.dart';
import 'package:{1}/src/features/{2}/domain/entities/{2}_entity.dart';
import 'package:{1}/src/features/{2}/domain/repositories/{2}_repository.dart';
import 'package:{1}/src/features/{2}/presentations/blocs/state/{2}_state.dart';
import 'package:{1}/src/shared/state/api_state.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class {0}Cubit extends Cubit<{0}State> {{
  final {0}Repository repo;

  {0}Cubit(this.repo) : super({0}State());

  Future<void> example() async {{
    try {{
      emit(state.copyWith(api: state.api.loading()));

      final {0}Entity result = await repo.example();

      emit(state.copyWith(api: state.api.success(), {2}: result));
    }} on ApiException catch (e) {{
      emit(state.copyWith(api: state.api.errorException(e)));
    }} on RepositoryException catch (e) {{
      emit(state.copyWith(api: state.api.errorException(e)));
    }}
  }}
}}"#, capitalize_first_letter(&*name), package_name, name.to_lowercase());

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\presentations\\blocs\\bloc", feature_name.to_lowercase()))
        .join(&format!("{}_cubit.dart", name.to_lowercase()));

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