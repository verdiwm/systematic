fn main() {
    pkg_config::Config::new()
        .atleast_version("255")
        .probe("libsystemd")
        .expect("Failed to link to libsystemd");
}
