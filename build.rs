use std::env;
use std::path::PathBuf;

fn main() {
    // If STEAM_SDK_LOCATION_REL is set, resolve it to an absolute path
    if let Ok(rel_path) = env::var("STEAM_SDK_LOCATION_REL") {
        let abs_path = {
            let path = PathBuf::from(&rel_path);
            if path.is_relative() {
                env::current_dir()
                    .expect("Failed to get current directory")
                    .join(path)
            } else {
                path
            }
        };

        // Output an environment override for downstream build scripts
        println!("cargo:rustc-env=STEAM_SDK_LOCATION={}", abs_path.display());
    }
}
