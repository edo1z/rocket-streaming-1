#[macro_use]
extern crate rocket;

use crate::rocket::futures::StreamExt;
use rocket::data::{Data, ToByteUnit};
use tokio_util::io::ReaderStream;

#[post("/", data = "<data>")]
async fn index(data: Data<'_>) -> std::io::Result<()> {
    let mut stream = ReaderStream::new(data.open(20.mebibytes()));
    let mut stream_contents: Vec<u8> = Vec::new();
    while let Some(chunk) = stream.next().await {
        stream_contents.extend_from_slice(&chunk?);
        println!("Received {} bytes", stream_contents.len());
    }
    println!("{:?}", stream_contents);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
