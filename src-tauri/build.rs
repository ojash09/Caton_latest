fn main() {
    println!("cargo:rerun-if-changed=.env");

    let contents = std::fs::read_to_string(".env").expect("Failed to read .env file");
    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }
}
