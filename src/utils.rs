pub fn html_checkbox_to_bool(value: &Option<String>, default_value: bool) -> bool {
    match value {
        Some(_) => true,
        None => default_value,
    }
}
