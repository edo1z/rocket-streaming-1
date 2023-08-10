#[macro_use] extern crate rocket;

use rocket::tokio;
use rocket::data::{Data, ToByteUnit};

#[post("/", data = "<data>")]
async fn index(data: Data<'_>) -> std::io::Result<()> {
    data.open(10.bytes()).stream_to(tokio::io::stdout()).await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
