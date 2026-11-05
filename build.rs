fn main() {
  napi_build::setup();

  // Link the addon like simulang-rs links its own binaries. On macOS that
  // weak-links frameworks older versions lack.
  let link_args = std::env::var("DEP_SIMULANG_LINK_ARGS")
    .expect("DEP_SIMULANG_LINK_ARGS is missing; simulang-rs is too old");
  for arg in link_args.split_whitespace() {
    println!("cargo::rustc-link-arg={arg}");
  }
}
