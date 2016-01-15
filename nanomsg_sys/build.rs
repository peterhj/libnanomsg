use std::env;
use std::path::{PathBuf};

fn main() {
  //println!("cargo:rustc-flags=-L ../nanomsg-0.8-beta/.libs -l static=nanomsg");
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let mut libs_path = PathBuf::from(&manifest_dir);
  libs_path.pop();
  libs_path.push("nanomsg-0.8-beta");
  libs_path.push(".libs");
  println!("cargo:rustc-link-search=native={}", libs_path.to_str().unwrap());
  println!("cargo:rustc-link-lib=static=nanomsg");
}
