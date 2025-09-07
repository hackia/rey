use std::{
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
    process::Stdio,
};
use tabled::{builder::Builder, settings::Style};

use crate::console::views::admin::{ok_clear, ok_command};

pub const WEB_MAIN: &str = "./front/web";
pub const ADMIN_MAIN: &str = "./front/admin";
pub const WEB_TEMPLATES: &str = "./templates/web";
pub const ADMIN_TEMPLATES: &str = "./templates/admin";
pub const ASSETS_WEB: &str = "./front/web/scss";
pub const ASSETS_ADMIN: &str = "./front/admin/scss";
pub const SCRIPTS_WEB: &str = "./front/web/ts";
pub const SCRIPTS_ADMIN: &str = "./front/admin/ts";
pub const TESTS_WEB: &str = "./front/web/tests";
pub const TESTS_ADMIN: &str = "./front/admin/tests";
pub const TESTS_UNIT: &str = "./tests/unit";
pub const TESTS_INTEGRATION: &str = "./tests/integration";
pub const TESTS_E2E: &str = "./tests/e2e";
pub const TESTS_BENCHMARK: &str = "./tests/benchmark";
pub const VIEW_EXT: &str = "html.tera";
pub const ASSETS_EXT: &str = "scss";
pub const SCRIPTS_EXT: &str = "ts";
pub const TESTS_EXT: &str = ".tests.ts";

pub enum ViewType {
    Web,
    Admin,
}

pub fn generate_scss_and_ts_base(view: ViewType) -> Result<(), std::io::Error> {
    let dir = match view {
        ViewType::Web => "web",
        ViewType::Admin => "admin",
    };

    let filename = match view {
        ViewType::Web => "web.scss",
        ViewType::Admin => "admin.scss",
    };
    let mut main_scss = File::create(format!("./front/{dir}/scss/{filename}"))?;
    let mut main_ts = File::create(format!("./front/{dir}/ts/index.ts"))?;
    main_scss.write_all(b"@import './_simba.scss';\n")?;
    main_scss.sync_all()?;
    main_ts.write_all(b"console.log('Hello, Simba!');\n")?;
    main_ts.sync_all()?;
    Ok(())
}

pub fn scan() {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scanner = ignore::WalkBuilder::new(&home)
        .hidden(false)
        .ignore(false)
        .git_ignore(false)
        .git_exclude(false)
        .parents(false)
        .build();
    let mut databases: Vec<String> = Vec::new();
    for result in scanner {
        if let Ok(entry) = result {
            let path = entry.path();
            if path.is_dir() && path.ends_with("front/web") {
                databases.push(
                    path.display()
                        .to_string()
                        .replace("/front/web", "")
                        .replace(&home, "~"),
                );
            }
        }
    }
    databases.sort();
    let mut builder = Builder::new();
    builder.push_record(["Rey Projects Found"]);
    for db in databases {
        builder.push_record([db]);
    }
    let mut table = builder.build();
    table.with(Style::modern());
    println!("{}", table);
}
pub fn init() -> Result<(), std::io::Error> {
    ok_clear("Initializing project...", true);
    create_dir_all(WEB_MAIN)?;
    create_dir_all(ADMIN_MAIN)?;
    create_dir_all(WEB_TEMPLATES)?;
    create_dir_all(ASSETS_WEB)?;
    create_dir_all(SCRIPTS_WEB)?;
    create_dir_all(TESTS_WEB)?;
    create_dir_all(ADMIN_TEMPLATES)?;
    create_dir_all(ASSETS_ADMIN)?;
    create_dir_all(SCRIPTS_ADMIN)?;
    create_dir_all(TESTS_ADMIN)?;
    create_dir_all(TESTS_INTEGRATION)?;
    create_dir_all(TESTS_E2E)?;
    create_dir_all(TESTS_UNIT)?;
    create_dir_all(TESTS_BENCHMARK)?;
    create_dir_all("static")?;
    create_dir_all("static/css")?;
    create_dir_all("static/js")?;
    create_dir_all("static/img")?;
    create_dir_all("static/fonts")?;
    create_dir_all("logs")?;
    generate_scss_and_ts_base(ViewType::Web)?;
    generate_scss_and_ts_base(ViewType::Admin)?;

    let mut rocket_toml = File::create("Rocket.toml")?;

    let mut hgignore = File::create(".hgignore")?;

    hgignore.write_all(b"syntax: glob\n/target\n/node_modules\n/front/web/node_modules\n/front/admin/node_modules\n.DS_Store\n")?;
    hgignore.sync_all()?;

    rocket_toml.write_all(b"[default]\naddress = \"0.0.0.0\"\nport = 8000\n")?;
    rocket_toml.sync_all()?;

    ok_command(
        "Initializing typescript project...",
        false,
        std::process::Command::new("tsc")
            .arg("--init")
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
    );
    ok_command(
        "Initializing npm project...",
        false,
        std::process::Command::new("npm")
            .arg("init")
            .arg("-y")
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
    );
    ok_command(
        "initializing project...",
        false,
        std::process::Command::new("cargo")
            .arg("init")
            .arg("--vcs")
            .arg("hg")
            .stdout(Stdio::null())
            .stderr(Stdio::null()),
    );
    Ok(())
}
pub fn again_init() {
    if is_initialized() {
        ok_clear("removing existing project files...", false);
        ok_clear("removing front directory...", false);
        remove_dir_all("front").expect("failed to remove front directory");
        ok_clear("removing templates directory...", false);
        remove_dir_all("templates").expect("failed to remove templates directory");
        ok_clear("existing project files removed!", true);
        ok_clear("re-initializing project...", false);
        init().expect("failed to re-initialize project");
        ok_clear("project re-initialized successfully!", true);
    } else {
        ok_clear("project is not initialized yet.", false);
        ok_clear("initializing project...", false);
        init().expect("failed to initialize project");
        ok_clear("project initialized successfully!", true);
    }
}

pub fn is_initialized() -> bool {
    std::path::Path::new(WEB_MAIN).exists()
        && std::path::Path::new(ADMIN_MAIN).exists()
        && std::path::Path::new(WEB_TEMPLATES).exists()
        && std::path::Path::new(ASSETS_WEB).exists()
        && std::path::Path::new(SCRIPTS_WEB).exists()
        && std::path::Path::new(TESTS_WEB).exists()
        && std::path::Path::new(ADMIN_TEMPLATES).exists()
        && std::path::Path::new(ASSETS_ADMIN).exists()
        && std::path::Path::new(SCRIPTS_ADMIN).exists()
        && std::path::Path::new(TESTS_ADMIN).exists()
}

pub fn init_all() -> Result<(), std::io::Error> {
    init()?;
    generate_web("index")?;
    generate_admin("dashboard")?;
    Ok(())
}

pub fn init_web_only() -> Result<(), std::io::Error> {
    create_dir_all(WEB_MAIN)?;
    create_dir_all(WEB_TEMPLATES)?;
    create_dir_all(ASSETS_WEB)?;
    create_dir_all(SCRIPTS_WEB)?;
    create_dir_all(TESTS_WEB)?;
    generate_web("index")?;
    Ok(())
}

pub fn init_admin_only() -> Result<(), std::io::Error> {
    create_dir_all(ADMIN_MAIN)?;
    create_dir_all(ADMIN_TEMPLATES)?;
    create_dir_all(ASSETS_ADMIN)?;
    create_dir_all(SCRIPTS_ADMIN)?;
    create_dir_all(TESTS_ADMIN)?;
    generate_admin("dashboard")?;
    Ok(())
}

pub fn view_web_path(name: &str) -> String {
    format!("{WEB_TEMPLATES}/{name}.{VIEW_EXT}")
}

pub fn asset_web_path(name: &str) -> String {
    format!("{ASSETS_WEB}/_{name}.{ASSETS_EXT}")
}

pub fn script_web_path(name: &str) -> String {
    format!("{SCRIPTS_WEB}/{name}.{SCRIPTS_EXT}")
}
pub fn test_web_path(name: &str) -> String {
    format!("{TESTS_WEB}/{name}_test.{TESTS_EXT}")
}

pub fn view_admin_path(name: &str) -> String {
    format!("templates/admin/{name}.{VIEW_EXT}")
}
pub fn asset_admin_path(name: &str) -> String {
    format!("{ASSETS_ADMIN}/_{name}.{ASSETS_EXT}")
}

pub fn script_admin_path(name: &str) -> String {
    format!("{SCRIPTS_ADMIN}/{name}.{SCRIPTS_EXT}")
}
pub fn test_admin_path(name: &str) -> String {
    format!("{TESTS_ADMIN}/{name}_test.{TESTS_EXT}")
}

pub fn init_web_if_not_initialized() {
    if is_initialized() == false {
        init_web_only().unwrap();
    }
}
/// Generate a template file for the web view.
pub fn generate_web_view(name: &str) -> Result<(), std::io::Error> {
    File::create(view_web_path(name).as_str())?;
    Ok(())
}

/// Generate an asset file for the web view.
pub fn generate_web_asset(name: &str) -> Result<(), std::io::Error> {
    File::create(asset_web_path(name).as_str())?;
    Ok(())
}

/// Generate a script file for the web view.
pub fn generate_web_script(name: &str) -> Result<(), std::io::Error> {
    File::create(script_web_path(name).as_str())?;
    Ok(())
}
/// Generate a test file for the web view.
pub fn generate_web_test(name: &str) -> Result<(), std::io::Error> {
    File::create(test_web_path(name).as_str())?;
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
    let tpl = view_web_path(name);
    let asset = asset_web_path(name);
    let script = script_web_path(name);
    let test = test_web_path(name);
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
    let old_tpl = view_web_path(old);
    let new_tpl = view_web_path(new);
    let old_asset = asset_web_path(old);
    let new_asset = asset_web_path(new);
    let old_script = script_web_path(old);
    let new_script = script_web_path(new);
    let old_test = test_web_path(old);
    let new_test = test_web_path(new);
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
    let entries = std::fs::read_dir(WEB_TEMPLATES)?;
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
    std::path::Path::new(&view_admin_path(name)).exists()
}

/// Check if a web view asset exists.
pub fn asset_admin_exists(name: &str) -> bool {
    std::path::Path::new(&asset_admin_path(name)).exists()
}

/// Check if a web view script exists.
pub fn script_admin_exists(name: &str) -> bool {
    std::path::Path::new(&script_admin_path(name)).exists()
}

/// Check if a web view test exists.
pub fn test_admin_exists(name: &str) -> bool {
    std::path::Path::new(&test_admin_path(name)).exists()
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
    File::create(view_admin_path(name))?;
    Ok(())
}

/// Generate an asset file for the admin view.
pub fn generate_admin_asset(name: &str) -> Result<(), std::io::Error> {
    File::create(asset_admin_path(name))?;
    Ok(())
}

/// Generate a script file for the admin view.
pub fn generate_admin_script(name: &str) -> Result<(), std::io::Error> {
    File::create(script_admin_path(name))?;
    Ok(())
}

/// Generate a test file for the admin view.
pub fn generate_admin_test(name: &str) -> Result<(), std::io::Error> {
    File::create(test_admin_path(name))?;
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
    let tpl = view_admin_path(name);
    let asset = asset_admin_path(name);
    let script = script_admin_path(name);
    let test = test_admin_path(name);
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
    let old_tpl = view_admin_path(old);
    let new_tpl = view_admin_path(new);
    let old_asset = asset_admin_path(old);
    let new_asset = asset_admin_path(new);
    let old_script = script_admin_path(old);
    let new_script = script_admin_path(new);
    let old_test = test_admin_path(old);
    let new_test = test_admin_path(new);

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
    let entries = std::fs::read_dir(ADMIN_TEMPLATES)?;
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
    std::path::Path::new(view_web_path(name).as_str()).exists()
}

/// Check if a web view asset exists.
pub fn asset_web_exists(name: &str) -> bool {
    std::path::Path::new(asset_web_path(name).as_str()).exists()
}

/// Check if a web view script exists.
pub fn script_web_exists(name: &str) -> bool {
    std::path::Path::new(script_web_path(name).as_str()).exists()
}

/// Check if a web view test exists.
pub fn test_web_exists(name: &str) -> bool {
    std::path::Path::new(test_web_path(name).as_str()).exists()
}

/// Check if all files associated with a admin view exist.
pub fn admin_exists(name: &str) -> bool {
    template_admin_exists(name)
        && asset_admin_exists(name)
        && script_admin_exists(name)
        && test_admin_exists(name)
}
