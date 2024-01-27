pub fn html_checkbox_to_bool(value: &Option<String>) -> bool {
    match value {
        Some(_) => true,
        None => false,
    }
}
