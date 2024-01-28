use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/static/"]
struct Asset;

pub fn get_embed_static_file(path: &str) -> Vec<u8> {
    match Asset::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![],
    }
}

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Template;
pub fn get_embed_template_file(path: &str) -> Vec<u8> {
    match Template::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![],
    }
}

