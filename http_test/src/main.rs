use reqwest::Proxy;

fn main(){
    run().unwrap();
}

fn run() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::builder()
        .proxy(Proxy::all("http://localhost:8888")?)
        .build()?;
    let mut res = client.get("https://www.rust-lang.org").send()?;
    let body = res.text()?;
    println!("{}", body);
    Ok(())
        
}

// fn main() {
//     run().unwrap();
// }

// fn run() -> Result<(), reqwest::Error> {
//     let client = reqwest::Client::builder()
//         .proxy(Proxy::all("http://localhost:8888")?)
//         .build()?;
//     let mut res = client.get("https://www.rust-lang.org").send()?;
//     let body = res.text()?;
//     println!("{}", body);
//     Ok(())
// }