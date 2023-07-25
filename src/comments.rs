pub fn get_comment_string(extension: String) -> String {
  if extension == "js".to_string() {
    "//".to_string()
  } else {
    "".to_string();
  }
}