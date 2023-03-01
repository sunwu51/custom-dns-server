mod client;
mod server;

#[tokio::main]
async fn main() {
    println!("{:?}", client::query_a_record("www.example.com").await);
    println!("{:?}", client::query_a_record("www.baidu.com").await);
    println!("{:?}", client::query_a_record("www.google.com").await);
}
