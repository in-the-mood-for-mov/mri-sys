use std::{env, path};

fn rb_config(key: &str) -> subprocess::Result<String> {
  subprocess::Exec::cmd("ruby")
    .args(&["-e", &format!(r#"print(RbConfig::CONFIG.fetch("{}"))"#, key)])
    .stdout(subprocess::Redirection::Pipe)
    .capture()
    .map(|capture| capture.stdout_str())
}

fn main() {
  let ruby_header_dir = rb_config("rubyhdrdir").unwrap();
  let ruby_arch_header_dir = rb_config("rubyarchhdrdir").unwrap();
  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .clang_arg("-Wno-ignored-pragma-optimize")
    .clang_arg("-Wno-ignored-attributes")
    .clang_arg("-Iinternal-headers/ruby-2.6")
    .clang_arg(format!("-I{}", ruby_header_dir))
    .clang_arg(format!("-I{}", ruby_arch_header_dir))
    .default_enum_style(bindgen::EnumVariation::ModuleConsts)
    .generate_comments(false)
    .whitelist_type("rb_iseq_constant_body")
    .whitelist_type("rb_iseq_constant_body")
    .whitelist_type("ibf_header")
    .generate()
    .unwrap();

  let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .unwrap();
}
