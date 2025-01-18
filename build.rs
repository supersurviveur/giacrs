use std::fs;

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        let mut files = vec![];
        println!("cargo::rerun-if-changed=wrapper");

        // Compiling wrapper
        let paths = fs::read_dir("wrapper").unwrap();
        for path in paths {
            let file = path.unwrap().path();
            if file.is_file() {
                match file.extension().unwrap().to_str().unwrap() {
                    "cpp" => {
                        println!("cargo::rerun-if-changed={}", file.to_str().unwrap());
                        files.push(file);
                    }
                    "hpp" => {
                        println!("cargo::rerun-if-changed={}", file.to_str().unwrap());
                    }
                    _ => {}
                }
            }
        }

        cc::Build::new()
            .cpp(true)
            .cpp_link_stdlib("stdc++")
            .files(files)
            .compile("giacrs_wrapper");

        // shared library.
        println!("cargo:rustc-link-lib=giac");
    }
}
