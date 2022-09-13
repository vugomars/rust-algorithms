pub fn string_rev(text: &str) -> String { text.chars().rev().collect() } // O(n)

#[cfg(test)]
mod test {
  use super::string_rev;

  #[test]
  fn test_rev_str() {
    let a = "Hello World";
    let result = string_rev(a);
    assert_eq!(result, "dlroW olleH".to_string());
  }
}
