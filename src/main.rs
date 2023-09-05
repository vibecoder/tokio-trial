use log::Level;
use tokio::io::AsyncReadExt;

async fn run() {
    tokio::join!(
        sleeper(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        reader(),
        writer(),
    );
}

async fn reader() {
    log::info!("This is the reader function");
    let mut f = tokio::fs::File::open("beeg.csv").await.unwrap();
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.unwrap();
    log::info!("Read beeg {} bytes", contents.len());
}

async fn sleeper() {
    log::info!("This is the sleeper function");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Awake");
}

async fn writer() {
    log::info!("This is the writer function");
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Took {:?} seconds", end - start);
}
