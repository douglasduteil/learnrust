extern crate cursive;

use cursive::{Cursive};
use cursive::views::Dialog;

pub struct WorkshopperOptions {
  pub title: &'static str,
}

pub struct Workshopper {
  options: WorkshopperOptions,
  siv: Cursive,
}

impl Workshopper {

  /// Creates a new instance of a Workshopper
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use std::string::{String};
  /// # use workshopper::{Workshopper, WorkshopperOptions};
  /// let workshop = Workshopper::new(
  ///   WorkshopperOptions {
  ///     title: "My workshop"
  ///   }
  /// )
  /// # ;
  /// ```
  pub fn new(options: WorkshopperOptions) -> Self {
    Workshopper {
      options: options,
      siv: Cursive::new()
    }
  }

  pub fn run(&mut self) {
    let dialog = Dialog::text("Did you do the thing?")
      .title(self.options.title);

    self.siv
      .add_layer(dialog)
    ;

    self.siv.run();
  }
}

pub fn lol(x: i32) -> i32 {
  x + 1
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
