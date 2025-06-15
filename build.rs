use dbc_codegen::{self, Config};

fn main() {
    let dbc_path = "cansea.dbc";
    let dbc_file = std::fs::read(dbc_path).unwrap();
    println!("cargo:rerun-if-changed={}", dbc_path);

    let config = Config::builder()
        .dbc_name("example.dbc")
        .dbc_content(&dbc_file)
        //.allow_dead_code(true) // Don't emit warnings if not all generated code is used
        //.impl_arbitrary(FeatureConfig::Gated("arbitrary")) // Optional impls.
        //.impl_debug(FeatureConfig::Always)                 // See rustdoc for more,
        //.check_ranges(FeatureConfig::Never)                // or look below for an example.
        .build();

    let mut out = std::io::BufWriter::new(std::fs::File::create("src/messages.rs").unwrap());
    dbc_codegen::codegen(config, &mut out).expect("dbc-codegen failed");
}
