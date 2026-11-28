/// Convert PascalCase to snake_case
pub fn pascal_to_snake(name: &str) -> String {
  let mut result = String::new();
  for (index, character) in name.chars().enumerate() {
    if character.is_uppercase() && index > 0 {
      result.push('_');
    }
    result.push(character.to_ascii_lowercase());
  }
  result
}
