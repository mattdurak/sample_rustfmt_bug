use std::fs::File;

fn main() {
    let name = "sample";
    let out_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");
    let in_path = env!("CARGO_MANIFEST_DIR");

    let input_file = std::path::Path::new(&in_path).join("template.rs");

    let mut in_f = File::open(&input_file)
        .unwrap_or_else(|e| panic!("Failed to open input file {}: {}", input_file.display(), e));

    println!("creating target file: {name}");
    let output_file = std::path::Path::new(&out_path).join(format!("{name}.rs"));
    {
        let mut out_f = File::create(&output_file).unwrap_or_else(|e| {
            panic!(
                "Failed to create target file {}: {}",
                output_file.display(),
                e
            )
        });

        std::io::copy(&mut in_f, &mut out_f).unwrap_or_else(|e| {
            panic!(
                "Failed to copy contents from {} to {}: {}",
                input_file.display(),
                output_file.display(),
                e
            )
        });
    }

    // Format the generated code using rustfmt
    println!("formatting generated code: {name}");
    let x = std::process::Command::new("rustfmt")
        .arg(output_file.to_str().unwrap())
        .arg("--verbose")
        .output()
        .unwrap_or_else(|_| panic!("failed to format {name}.rs"));
    println!(
        "cargo::warning=rustfmt output: {}",
        String::from_utf8_lossy(&x.stdout)
    );
    println!(
        "cargo::warning=rustfmt err: {}",
        String::from_utf8_lossy(&x.stderr)
    );
}
