use std::fs::{File, create_dir_all};

use tabled::{builder::Builder, settings::Style};

/// Generate a template file for the web view.
pub fn generate_web_view(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("templates/web")?;
    let tpl = format!("templates/web/{name}.html.tera");
    File::create(tpl)?;
    Ok(())
}
/// Generate an asset file for the web view.
pub fn generate_web_asset(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("assets/web")?;
    let asset = format!("assets/web/{name}.scss");
    File::create(asset)?;
    Ok(())
}

/// Generate a script file for the web view.
pub fn generate_web_script(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("scripts/web")?;
    let script = format!("scripts/web/{name}.ts");
    File::create(script)?;
    Ok(())
}
/// Generate a test file for the web view.
pub fn generate_web_test(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("tests/web")?;
    let test = format!("tests/web/{name}_test.ts");
    File::create(test)?;
    Ok(())
}

/// Generate all files associated with a web view, including template, asset, script, and test files.
pub fn generate_web(name: &str) -> Result<(), std::io::Error> {
    generate_web_view(name)?;
    generate_web_asset(name)?;
    generate_web_script(name)?;
    generate_web_test(name)?;
    Ok(())
}

/// Remove all files associated with a web view, including template, asset, script, and test files.
pub fn remove_web(name: &str) -> Result<(), std::io::Error> {
    let tpl = format!("templates/web/{name}.html.tera");
    let asset = format!("assets/web/{name}.scss");
    let script = format!("scripts/web/{name}.ts");
    let test = format!("tests/web/{name}_test.ts");
    if std::path::Path::new(&tpl).is_file() {
        std::fs::remove_file(tpl)?;
    }
    if std::path::Path::new(&asset).is_file() {
        std::fs::remove_file(asset)?;
    }
    if std::path::Path::new(&script).is_file() {
        std::fs::remove_file(script)?;
    }
    if std::path::Path::new(&test).is_file() {
        std::fs::remove_file(test)?;
    }
    Ok(())
}
/// Rename all files associated with a web view, including template, asset, script, and test files.
pub fn rename_web(old: &str, new: &str) -> Result<(), std::io::Error> {
    let old_tpl = format!("templates/web/{old}.html.tera");
    let new_tpl = format!("templates/web/{new}.html.tera");
    let old_asset = format!("assets/web/{old}.scss");
    let new_asset = format!("assets/web/{new}.scss");
    let old_script = format!("scripts/web/{old}.ts");
    let new_script = format!("scripts/web/{new}.ts");
    let old_test = format!("tests/web/{old}_test.ts");
    let new_test = format!("tests/web/{new}_test.ts");
    if std::path::Path::new(&old_tpl).is_file() {
        std::fs::rename(old_tpl, new_tpl)?;
    }
    if std::path::Path::new(&old_asset).is_file() {
        std::fs::rename(old_asset, new_asset)?;
    }
    if std::path::Path::new(&old_script).is_file() {
        std::fs::rename(old_script, new_script)?;
    }
    if std::path::Path::new(&old_test).is_file() {
        std::fs::rename(old_test, new_test)?;
    }
    Ok(())
}

/// Get all web view templates available.
pub fn list_web() -> Result<Vec<String>, std::io::Error> {
    let mut views = Vec::new();
    let entries = std::fs::read_dir("templates/web")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "tera" {
                    if let Some(stem) = path.file_stem() {
                        if let Some(stem_str) = stem.to_str() {
                            let name = stem_str.trim_end_matches(".html");
                            views.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    Ok(views)
}
/// Print all web view templates in a table format.
pub fn print_web_templates() {
    match list_web() {
        Ok(templates) => {
            let mut builder = Builder::new();
            for template in templates {
                builder.push_record([template]);
            }
            let mut table = builder.build();
            table.with(Style::modern());
            println!("{table}");
        }
        Err(err) => {
            eprintln!("Error listing web view templates: {err}");
        }
    }
}

/// Check if a web view template exists.
pub fn template_admin_exists(name: &str) -> bool {
    let tpl = format!("templates/admin/{name}.html.tera");
    std::path::Path::new(&tpl).exists()
}

/// Check if a web view asset exists.
pub fn asset_admin_exists(name: &str) -> bool {
    let asset = format!("assets/admin/{name}.scss");
    std::path::Path::new(&asset).exists()
}

/// Check if a web view script exists.
pub fn script_admin_exists(name: &str) -> bool {
    let script = format!("scripts/admin/{name}.ts");
    std::path::Path::new(&script).exists()
}

/// Check if a web view test exists.
pub fn test_admin_exists(name: &str) -> bool {
    let test = format!("tests/admin/{name}_test.ts");
    std::path::Path::new(&test).exists()
}

/// Check if all files associated with a web view exist.
pub fn web_exists(name: &str) -> bool {
    template_web_exists(name)
        && asset_web_exists(name)
        && script_web_exists(name)
        && test_web_exists(name)
}

/// Generate a template file for the admin view.
pub fn generate_admin_view(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("templates/admin")?;
    let tpl = format!("templates/admin/{name}.html.tera");
    File::create(tpl)?;
    Ok(())
}

/// Generate an asset file for the admin view.
pub fn generate_admin_asset(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("assets/admin")?;
    let asset = format!("assets/admin/{name}.scss");
    File::create(asset)?;
    Ok(())
}

/// Generate a script file for the admin view.
pub fn generate_admin_script(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("scripts/admin")?;
    let script = format!("scripts/admin/{name}.ts");
    File::create(script)?;
    Ok(())
}

/// Generate a test file for the admin view.
pub fn generate_admin_test(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("tests/admin")?;
    let test = format!("tests/admin/{name}_test.ts");
    File::create(test)?;
    Ok(())
}

/// Generate all files associated with a admin view, including template, asset, script, and test files.
pub fn generate_admin(name: &str) -> Result<(), std::io::Error> {
    generate_admin_view(name)?;
    generate_admin_asset(name)?;
    generate_admin_script(name)?;
    generate_admin_test(name)?;
    Ok(())
}

/// Remove all files associated with a admin view, including template, asset, script, and test files.
pub fn remove_admin(name: &str) -> Result<(), std::io::Error> {
    let tpl = format!("templates/admin/{name}.html.tera");
    let asset = format!("assets/admin/{name}.scss");
    let script = format!("scripts/admin/{name}.ts");
    let test = format!("tests/admin/{name}_test.ts");
    if std::path::Path::new(&tpl).is_file() {
        std::fs::remove_file(tpl)?;
    }
    if std::path::Path::new(&asset).is_file() {
        std::fs::remove_file(asset)?;
    }
    if std::path::Path::new(&script).is_file() {
        std::fs::remove_file(script)?;
    }
    if std::path::Path::new(&test).is_file() {
        std::fs::remove_file(test)?;
    }
    Ok(())
}
/// Rename all files associated with a admin view, including template, asset, script, and test files.
pub fn rename_admin(old: &str, new: &str) -> Result<(), std::io::Error> {
    let old_tpl = format!("templates/admin/{old}.html.tera");
    let new_tpl = format!("templates/admin/{new}.html.tera");
    let old_asset = format!("assets/admin/{old}.scss");
    let new_asset = format!("assets/admin/{new}.scss");
    let old_script = format!("scripts/admin/{old}.ts");
    let new_script = format!("scripts/admin/{new}.ts");
    let old_test = format!("tests/admin/{old}_test.ts");
    let new_test = format!("tests/admin/{new}_test.ts");
    if std::path::Path::new(&old_tpl).is_file() {
        std::fs::rename(old_tpl, new_tpl)?;
    }
    if std::path::Path::new(&old_asset).is_file() {
        std::fs::rename(old_asset, new_asset)?;
    }
    if std::path::Path::new(&old_script).is_file() {
        std::fs::rename(old_script, new_script)?;
    }
    if std::path::Path::new(&old_test).is_file() {
        std::fs::rename(old_test, new_test)?;
    }
    Ok(())
}

/// Get all admin view templates available.
pub fn list_admin() -> Result<Vec<String>, std::io::Error> {
    let mut views = Vec::new();
    let entries = std::fs::read_dir("templates/admin")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "tera" {
                    if let Some(stem) = path.file_stem() {
                        if let Some(stem_str) = stem.to_str() {
                            let name = stem_str.trim_end_matches(".html");
                            views.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    Ok(views)
}
/// Print all admin view templates in a table format.
pub fn print_admin_templates() {
    match list_admin() {
        Ok(templates) => {
            let mut builder = Builder::new();
            for template in templates {
                builder.push_record([template]);
            }
            let mut table = builder.build();
            table.with(Style::modern());
            println!("{table}");
        }
        Err(err) => {
            eprintln!("Error listing web view templates: {err}");
        }
    }
}

/// Check if a web view template exists.
pub fn template_web_exists(name: &str) -> bool {
    let tpl = format!("templates/web/{name}.html.tera");
    std::path::Path::new(&tpl).exists()
}

/// Check if a web view asset exists.
pub fn asset_web_exists(name: &str) -> bool {
    let asset = format!("assets/web/{name}.scss");
    std::path::Path::new(&asset).exists()
}

/// Check if a web view script exists.
pub fn script_web_exists(name: &str) -> bool {
    let script = format!("scripts/web/{name}.ts");
    std::path::Path::new(&script).exists()
}

/// Check if a web view test exists.
pub fn test_web_exists(name: &str) -> bool {
    let test = format!("tests/web/{name}_test.ts");
    std::path::Path::new(&test).exists()
}

/// Check if all files associated with a admin view exist.
pub fn admin_exists(name: &str) -> bool {
    template_admin_exists(name)
        && asset_admin_exists(name)
        && script_admin_exists(name)
        && test_admin_exists(name)
}
