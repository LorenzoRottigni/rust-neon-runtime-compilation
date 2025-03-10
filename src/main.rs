use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    // Path to temporary directory
    let temp_dir = "path_to_temp_project";

    // Step 1: Store Rust code in a variable
    let rust_code = r#"
        use neon::prelude::*;

        fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
            Ok(cx.string("Hello from Rust at runtime!"))
        }

        #[neon::main]
        fn main(mut cx: ModuleContext) -> NeonResult<()> {
            cx.export_function("hello", hello)?;
            Ok(())
        }
    "#;

    // Step 2: Write the code to a temporary lib.rs
    std::fs::create_dir_all(format!("{}/src", temp_dir)).unwrap();
    std::fs::write(format!("{}/src/lib.rs", temp_dir), rust_code).unwrap();

    // Step 3: Generate a temporary Cargo.toml
    let cargo_toml = r#"
    [package]
    name = "neon_temp"
    version = "0.1.0"
    edition = "2021"

    [lib]
    crate-type = ["cdylib"]
    
    [dependencies]
    neon = "1"
    "#;
    std::fs::write(format!("{}/Cargo.toml", temp_dir), cargo_toml).unwrap();

    // Step 4: Compile the project using cargo
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(temp_dir)
        .output()
        .expect("Failed to run cargo build");

    if !output.status.success() {
        eprintln!(
            "❌ Build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    println!("✅ Compilation complete! Load it in Node.js with:");
    println!("require(\"{}/target/release/neon_temp.node\")", temp_dir);
}
