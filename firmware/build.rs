fn main() {
    embuild::espidf::sysenv::output();
    built::write_built_file().expect("Failed to acquire build-time information");
}
