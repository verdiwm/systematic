fn main() {
    pkg_config::Config::new()
        .atleast_version("235")
        .probe("libsystemd")
        .expect("Failed to link to libsystemd");
}
