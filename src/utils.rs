pub fn html_checkbox_to_bool(value: &Option<String>, defaultValue: bool) -> bool {
    match value {
        Some(_) => true,
        None => defaultValue,
    }
}
