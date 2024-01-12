use std::env;

fn main() {
    let domain_api = env::var("DOMAIN_API").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

    println!("cargo:rustc-env=DOMAIN_API={}", domain_api);
}
