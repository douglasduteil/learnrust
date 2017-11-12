extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
  let mut cg = includedir_codegen::start("FILES");

  cg.dir("exercises", Compression::Gzip)
    .build("exercises.rs")
    .unwrap();
}
