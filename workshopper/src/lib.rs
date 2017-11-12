extern crate cursive;

use cursive::{Cursive};
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};

pub struct WorkshopperOptions {
  pub title: &'static str,
  pub subtitle: &'static str,
  pub exercises: Vec<String>,
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
  ///     title: "My workshop",
  ///     subtitle: "To learn sth",
  ///     exercises: vec![
  ///       "exercises/hello_world/problem.md".to_string()
  ///     ],
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
    let titles = get_exercice_titles(&self.options.exercises);

    let mut select = SelectView::new();
    select.add_all_str(titles);

    let subtitle = TextView::new(self.options.subtitle);
    let dialog = Dialog::around(
      LinearLayout::vertical()
        .child(subtitle)
        .child(select)
    )
    .title(self.options.title);

    self.siv
      .add_layer(dialog)
    ;

    self.siv.run();
  }
}

pub fn get_exercice_titles (file_names: &Vec<String>) -> Vec<String> {
  let mut titles: Vec<String> = file_names.into_iter()
    .map(|x| String::from(x.split('/').nth(1).unwrap()))
    .collect()
  ;

  titles.dedup();

  titles
}
