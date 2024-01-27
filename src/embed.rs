use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/static/"]
struct Asset;

pub fn get_embed_file(path: &str) -> Vec<u8> {
    match Asset::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![],
    }
}
