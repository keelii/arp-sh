fn main() {
    #[cfg(feature = "bundled")]
    {
        minijinja_embed::embed_templates!("src/templates", &[".twig"]);
        println!("loaded templates.");
    }
}
