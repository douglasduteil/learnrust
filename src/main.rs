extern crate workshopper;
extern crate includedir;
extern crate phf;

use workshopper::{Workshopper, WorkshopperOptions};

include!(concat!(env!("OUT_DIR"), "/exercies.rs"));

fn main() {
  let file_names = includedir_filenames_to_vec_string(FILES.file_names());
  let exercises = file_names_to_exercices(&file_names);
  assert_eq!(exercises, ["01_hello-world", "02_hello-you"]);

  let mut workshop = Workshopper::new(WorkshopperOptions {
    title: "~ Wanna learn Rust ? ~",
    subtitle: "~ Here is some basic exercices ~",
  });

  workshop.run();
}

fn file_names_to_exercices (file_names: &Vec<String>) -> Vec<String> {
  file_names.into_iter()
    .map(|x| String::from(x.split('/').nth(1).unwrap()))
    .collect()
}

fn includedir_filenames_to_vec_string(file_names: includedir::FileNames) -> Vec<String> {
  file_names.map(|x| String::from(x))
    .collect::<Vec<String>>()
    // HACK(douglasduteil): revert includedir file_names result
    // includedir::FileNames seems to be inversed compared to the original file
    // order. Has the trait `std::iter::DoubleEndedIterator` is not implemented
    // for `includedir::FileNames`, `.rev()` is not accessible in the above
    // Iterator. Therefor I'm recreating an iterator from `std::vec::Vec` that
    // actually has it :-1:.
    .into_iter()
    .rev()
    .collect::<Vec<String>>()
}
