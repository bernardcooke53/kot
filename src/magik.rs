/// Firmly insert object_to_insert_firmly into recipient
pub fn with_consent(recipient: String, object_to_insert_firmly: String) -> String {
    recipient
        .chars()
        .map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => object_to_insert_firmly.clone(),
            'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => {
                object_to_insert_firmly.to_ascii_uppercase().clone()
            }
            _ => c.to_string(),
        })
        .collect()
}
