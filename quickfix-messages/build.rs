use quickfix_spec_generator::Builder;
use std::{env, fs};

fn main() {
    let builder = Builder::new()
        .add_path("../protocol-spec/FIX40.xml")
        .add_path("../protocol-spec/FIX41.xml")
        .add_path("../protocol-spec/FIX42.xml")
        .add_path("../protocol-spec/FIX43.xml")
        .add_path("../protocol-spec/FIX44.xml")
        .add_path("../protocol-spec/FIXT11.xml")
        .add_path("../protocol-spec/TEST_SPEC.xml")
        .enable_rustfmt(true);

    fs::create_dir_all("./out-preview").unwrap();
    builder.build("./out-preview").unwrap();
    builder.build(&env::var("OUT_DIR").unwrap()).unwrap();
}
