use quickfix_spec_generator::Builder;
use std::env;

fn main() {
    let builder = Builder::new()
        .add_path("../protocol-spec/FIX40.xml")
        .add_path("../protocol-spec/FIX41.xml")
        .add_path("../protocol-spec/FIX42.xml")
        .add_path("../protocol-spec/FIX43.xml")
        .add_path("../protocol-spec/FIX44.xml")
        .add_path("../protocol-spec/FIXT11.xml");

    builder.build("./test-out").unwrap();
    builder.build(&env::var("OUT_DIR").unwrap()).unwrap();
}
