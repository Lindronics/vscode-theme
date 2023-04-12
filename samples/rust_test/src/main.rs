#[tokio::main]
pub async fn main() {
    #[allow(clippy::disallowed_names)]
    let foo = "bar";

    for _ in 1..100 {
        println!("Hello {foo}");
    }

    let x = 5;
    match fizz(x).await {
        Some(_buzz) => {
            // asdf
            todo!()
        }
        None => {
            todo!()
        }
    };
}

/// does some stuff
async fn fizz(_x: u16) -> Option<Buzz<u16>> {
    None
}

#[derive(Clone)]
struct Buzz<T>
where
    T: Clone,
{
    _inner: T,
}
