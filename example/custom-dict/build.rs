use quickfix_spec_generator::Builder;
use std::{env, fs};

fn main() {
    let builder = Builder::new().add_path("./ZE_CORP.xml");

    fs::create_dir_all("./out-preview").unwrap();
    builder.build("./out-preview").unwrap();
    builder.build(&env::var("OUT_DIR").unwrap()).unwrap();
}
