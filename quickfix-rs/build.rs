use quickfix_spec_generator::Builder;

fn main() {
    Builder::new()
        .add_path("../protocol-spec/FIX40.xml")
        .add_path("../protocol-spec/FIX41.xml")
        .add_path("../protocol-spec/FIX42.xml")
        .add_path("../protocol-spec/FIX43.xml")
        .add_path("../protocol-spec/FIX44.xml")
        .add_path("../protocol-spec/FIXT11.xml")
        .build("./test-out")
        .unwrap();
}
