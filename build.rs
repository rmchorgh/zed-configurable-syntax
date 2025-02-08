include!("src/recompile.rs");

fn main() {
    match recompile(false) {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}
