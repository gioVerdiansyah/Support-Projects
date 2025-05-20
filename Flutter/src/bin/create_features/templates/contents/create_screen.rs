use std::io;
use std::path::PathBuf;
use Flutter::utils::create_file::create_file_content;
use Flutter::utils::global_args::get_args;
use Flutter::utils::string::capitalize_first_letter;

pub fn create_screen() -> io::Result<()> {
    let args = get_args();
    let feature_name = args.get(0).cloned().unwrap_or_default();
    let package_name = args.get(1).cloned().unwrap_or_default();

    let content = &format!(r#"
import 'package:flutter/material.dart';
import 'package:{1}/src/shared/constants/color_constant.dart';

class {0}Screen extends StatefulWidget {{
  const {0}Screen({{super.key}});

  @override
  State<StatefulWidget> createState() => _{0}State();
}}

class _{0}State extends State<{0}Screen> {{

    @override
    void initState() {{
        super.initState();

    }}

    @override
    void dispose() {{
        super.dispose();

    }}

    @override
    Widget build(BuildContext context) {{
        return Scaffold(
            backgroundColor: ColorConstant.primary,
            child: Center(
                child: Text("Hello {0} Screen")
            )
        );
    }}
}}"#, capitalize_first_letter(&*feature_name), package_name);

    let file_path = PathBuf::from(format!("lib\\src\\features\\{}\\presentations\\screens", feature_name.to_lowercase()))
        .join(&format!("{}_screen.dart", feature_name.to_lowercase()));

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