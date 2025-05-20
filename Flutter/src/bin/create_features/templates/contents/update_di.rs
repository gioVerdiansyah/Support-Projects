use std::io;
use Flutter::utils::files::insert_content_to_file;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn update_di(only_modules: Option<Vec<&str>>) -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();

    let class_name = capitalize_first_letter(&feature_name);
    let lower_name = feature_name.to_lowercase();

    let import_code = format!(
        r#"
import 'package:{0}/src/features/{1}/data/api/{1}_api.dart';
import 'package:{0}/src/features/{1}/data/repositories/{1}_repository_impl.dart';
import 'package:{0}/src/features/{1}/domain/repositories/{1}_repository.dart';
import 'package:{0}/src/features/{1}/presentations/blocs/bloc/{1}_cubit.dart';
"#,
        package_name, lower_name
    );

    let api_code = format!(
        r#"
    getIt.registerFactory<{0}Api>(
        () => {0}Api(getIt<ApiClient>()),
    );
    "#,
        class_name
    );

    let repo_code = format!(
        r#"
    getIt.registerLazySingleton<{0}Repository>(
        () => {0}RepositoryImpl(getIt<{0}Api>()),
    );
    "#,
        class_name
    );

    let bloc_code = format!(
        r#"
    getIt.registerFactory<{0}Cubit>(
        () => {0}Cubit(getIt<{0}Repository>()),
    );
    "#,
        class_name
    );

    let di_path = "lib\\src\\core\\di\\injection.dart";

    let insert = |marker: &str, code: &str, label: &str| {
        if let Err(e) = insert_content_to_file(di_path, marker, code) {
            eprintln!("Failed update di.dart > {}: {}", label, e);
        }
    };

    match only_modules {
        Some(modules) => {
            for m in &modules {
                match m {
                    &"api" => insert("//! API", &api_code, "API"),
                    &"repository" | &"repository_impl" => insert("//! Repositories", &repo_code, "Repository"),
                    &"cubit" | &"bloc" => insert("//! Blocs", &bloc_code, "Bloc"),
                    &"model" | &"entity" | &"state" | &"screen" => {}
                    _ => {
                        eprintln!("Unknown module '{}'", m);
                    }
                }
            }
            if modules.iter().any(|m| ["api", "repository", "repository_impl", "cubit", "bloc"].contains(m)) {
                insert("import 'package:coffee_shop/src/core/system/auth_local.dart';", &import_code, "Import");
            }
        }
        None => {
            insert("import 'package:coffee_shop/src/core/system/auth_local.dart';", &import_code, "Import");
            insert("//! API", &api_code, "API");
            insert("//! Repositories", &repo_code, "Repository");
            insert("//! Blocs", &bloc_code, "Bloc");
        }
    }

    Ok(())
}