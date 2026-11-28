/// Convert snake_case to PascalCase
pub fn snake_to_pascal(name: &str) -> String {
  name
    .split('_')
    .map(|word| {
      let mut chars = word.chars();
      match chars.next() {
        Some(first) => {
          let upper: String = first.to_uppercase().collect();
          upper + chars.as_str()
        }
        None => String::new(),
      }
    })
    .collect()
}
