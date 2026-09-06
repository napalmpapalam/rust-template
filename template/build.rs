//! Emits `built.rs` with build-time metadata, read by `src/version.rs`.

fn main() {
    // Container builds have no `.git` (see .dockerignore) and pass the commit
    // in this way instead.
    println!("cargo::rerun-if-env-changed=GIT_COMMIT");

    // Degrade instead of failing the build: without either source the version
    // output falls back to "unknown".
    if let Err(err) = built::write_built_file() {
        println!("cargo::warning=failed to write build metadata: {err}");
    }
}
