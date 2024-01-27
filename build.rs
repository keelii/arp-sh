fn main() {
    #[cfg(feature = "bundled")]
    {
        minijinja_embed::embed_templates!("src/templates");
        println!("loaded templates.");
    }
}
