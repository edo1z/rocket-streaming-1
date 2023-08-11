#[macro_use]
extern crate rocket;

use rocket::data::{Data, DataStream, ToByteUnit};

#[post("/", data = "<data>")]
async fn index(data: Data<'_>) -> std::io::Result<()> {
    let stream: DataStream = data.open(20.mebibytes());
    let hoge = stream.into_bytes().await?;
    println!("bytes => {:?}", hoge.into_inner());
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
