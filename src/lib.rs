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
    dir: &Path,
  ) -> Result<String, String> {
    match env::current_dir() {
      Ok(ref pwd) => env::set_var("OLDPWD", pwd),
      Err(_) => return Err("Please, checks your PWD variable\n".to_string()),
    };
    match env::set_current_dir(&dir) {
      Ok(_) => {
        match env::current_dir() {
          Ok(ref pwd) => {
            env::set_var("PWD", pwd);
            Ok("".to_string())
          },
          Err(_) => panic!("PWD corrupted\n"),
        }
      },
      Err(why) => Err(why.to_string()),
    }
  }

  /// The `from_arg` function returns empty or error's text information.
  pub fn from_arg (
    dir: &String,
  ) -> Result<String, String> {
    match env::var("OLDPWD") {
      Ok(old) => {
        match dir {
          d if d == "~" => match env::home_dir() {
            Some(ref home) => set(home),
            None => Err("Please, checks your HOME variable\n".to_string()),
          },
          d if d == "-" => set(Path::new(&old)),
          _ => set(Path::new(dir)),
        }
      },
      Err(_) => return Err("Please, checks your OLDPWD variable\n".to_string()),
    }
  }

  /// The `from_args` function returns empty or error's text information
  /// for first problem encounter.
  pub fn from_args (
    dirs: &Vec<String>,
  ) -> String {
    let mut out:String = String::new();

    for dir in dirs {
      match from_arg(dir) {
        Ok(val) => out.push_str(&val),
        Err(why) => return why,
      }
    }
    out
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
}
