// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/cd/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cdlib;

fn main() {
  let mut line:String = String::new();
  loop {
    match std::io::Write::write(
      &mut std::io::stderr(),
      &cdlib::cd::display().into_bytes(),
    ) {
      Ok(_) => {},
      Err(why) => panic!("Unable to write to stderr: {}", why),
    };
    let out = match std::io::stdin().read_line(&mut line) {
      Ok(_) => {
        match cdlib::cd::from_arg(&line.chars().take_while(|x|
          *x != '\n'
        ).collect()) {
          Ok(_) => {},
          Err(why) => why,
        }
      },
      Err(_) => break ,
    };
    match std::io::Write::write(
      &mut std::io::stderr(),
      &out.into_bytes(),
    ) {
      Ok(_) => line.clear(),
      Err(why) => panic!("Unable to write to stderr: {}", why),
    };
  }
}
