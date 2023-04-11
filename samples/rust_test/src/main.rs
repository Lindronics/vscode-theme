#[tokio::main]
pub async fn main() {
    #[allow(clippy::disallowed_names)]
    let foo = "bar";
    println!("Hello {foo}");
    fizz();
}

fn fizz() -> Option<Buzz> {
    None
}

struct Buzz {}
