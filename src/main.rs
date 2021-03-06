extern crate includedir;
extern crate phf;
extern crate workshopper;

use workshopper::{Workshopper, WorkshopperOptions};

include!(concat!(env!("OUT_DIR"), "/exercises.rs"));

fn main() {
  let exercises = includedir_filenames_to_vec_string(FILES.file_names());

  let mut workshop = Workshopper::new(WorkshopperOptions {
    title: "~ Wanna learn Rust ? ~",
    subtitle: "~ Here is some basic exercises ~",
    exercises: exercises,
  });

  workshop.run();
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
