extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
  let mut cg = includedir_codegen::start("FILES");

  cg.dir("exercies", Compression::Gzip)
    .build("exercies.rs")
    .unwrap();
}
