// Drops the files the answers turned off, then prints what to do next.

let license = variable::get("license");
if license != "MIT" && license != "MIT OR Apache-2.0" {
    file::delete("LICENSE-MIT");
}
if license != "Apache-2.0" && license != "MIT OR Apache-2.0" {
    file::delete("LICENSE-APACHE");
}

let ci = variable::get("ci");
if ci != "github" {
    file::delete(".github");
}
if ci != "gitlab" {
    file::delete(".gitlab-ci.yml");
}

if !variable::get("docker") {
    file::delete("Dockerfile");
    file::delete(".dockerignore");
}

if !variable::get("githooks") {
    file::delete(".githooks");
}

if !variable::get("claude") {
    file::delete("CLAUDE.md");
    file::delete(".claude");
}

// The server is the one crate that ships; without it the workspace starts empty.
if !variable::get("server") {
    file::delete("crates");
}

let name = variable::get("project-name");
print("");
print("  " + name + " is ready.");
print("");
print("    cd " + name);
print("    cargo run -- run all");
print("");
print("  That first build writes Cargo.lock. Commit it — CI runs --locked.");
print("");
if variable::get("githooks") {
    print("  Hooks are not installed by clone. Turn them on once:");
    print("    git config core.hooksPath .githooks");
    print("");
}
