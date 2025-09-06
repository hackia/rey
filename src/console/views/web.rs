use std::fs::{File, create_dir_all};

pub fn generate_web_view(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("templates/web")?;
    let tpl = format!("templates/web/{name}.html.tera");
    File::create(tpl)?;
    Ok(())
}

pub fn generate_web_asset(name: &str) -> Result<(), std::io::Error> {
    create_dir_all("assets/web")?;
    let asset = format!("assets/web/{name}.scss");
    File::create(asset)?;

    Ok(())
}
