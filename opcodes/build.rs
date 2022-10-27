use config_struct::create_struct;
use config_struct::DynamicLoading;
use config_struct::IntSize;
use config_struct::StructOptions;

fn main() {
    create_struct(
        "opcodes.toml",
        "src/opcodes.rs",
        &StructOptions {
            struct_name: String::from("Opcodes"),
            default_int_size: IntSize::I8,
            generate_load_fns: false,
            dynamic_loading: DynamicLoading::Never,
            ..StructOptions::default()
        },
    )
    .expect("err loading opcodes");
}
