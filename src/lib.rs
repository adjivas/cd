// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/cd/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod cd {
  use std::env;
  use std::path::Path;

  /// The `set` function returns try to changes te current repertory.
  fn set (
    dir: &String,
  ) -> Result<String, String> {
    let root = Path::new(dir);

    match env::set_current_dir(&root) {
      Ok(_) => Ok("".to_string()),
      Err(why) => Err(why.to_string()),
    }
  }

  /// The `display` function returns the current address.
  pub fn display (
  ) -> String {
    match env::current_dir() {
      Ok(pwd) => {
        let mut current = pwd.display().to_string();

        current.push_str(" ");
        current
      },
      Err(why) => why.to_string(),
    }
  }

  /// The `interpreter` function returns empty or error's text information.
  pub fn interpreter (
    dir: &String
  ) -> String {
    match set(dir) {
      Ok(val) => val,
      Err(mut why) => {
        why.push_str("\n");
        why
      },
    }
  }
}
