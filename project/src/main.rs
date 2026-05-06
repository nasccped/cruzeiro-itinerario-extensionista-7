fn turn_green(x: &str) -> String {
    format!("\x1b[92m{}\x1b[0m", x)
}

fn main() {
    println!("Server is {}!", turn_green("ready to run"));
}
