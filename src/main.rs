use generic_subdomains::file::download;

#[tokio::main]
async fn main() {
    match download("https://doc.rust-jp.rs", "test.html").await {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    };
}
