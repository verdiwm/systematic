fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    pkg_config::Config::new()
        .atleast_version("235")
        .probe("libsystemd")
        .expect("Failed to link to libsystemd");
}
