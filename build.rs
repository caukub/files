use std::io::Write;
use std::path::PathBuf;
use std::{fs, fs::File};

fn main() {
    const DIR: &'static str = env!("CARGO_MANIFEST_DIR");
    const FRONTEND_DIR: &'static str = "frontend";

    const INPUT_FILE: &'static str = "input.css";
    const OUTPUT_FILE: &'static str = "style.css";

    let frontend_path = PathBuf::from(DIR).join(FRONTEND_DIR);

    let input = frontend_path.join(INPUT_FILE);
    let output = frontend_path.join(OUTPUT_FILE);

    if !fs::exists(&input).unwrap() {
        let mut file = File::create(&input).unwrap();
        file.write_all(r#"@import "tailwindcss";"#.as_bytes())
            .unwrap();
    }
    if !fs::exists(&output).unwrap() {
        File::create(&output).unwrap();
    }

    const EXECUTABLE: &'static str = if cfg!(target_os = "windows") {
        "tailwindcss.exe"
    } else {
        "tailwindcss"
    };

    let result = std::process::Command::new(&format!("./{FRONTEND_DIR}/{EXECUTABLE}"))
        .args([
            "-i",
            &input.display().to_string(),
            "-o",
            &output.display().to_string(),
            "--minify",
        ])
        .output()
        .expect("Failed to run TailwindCSS executable");

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo:warning=Unable to build CSS: {error}");
    }
}
