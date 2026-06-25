use std::{fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    rerun_if_changed(Path::new("migrations"));
}

fn rerun_if_changed(path: &Path) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            rerun_if_changed(&path);
        } else {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
