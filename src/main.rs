#[macro_use] extern crate rocket;

use rocket::tokio;
use rocket::data::{Data, ToByteUnit};

#[post("/", data = "<data>")]
async fn index(data: Data<'_>) -> std::io::Result<()> {
    // 最大20MBまで受け取るストリームを作成し、受け取ったデータを順次標準出力に書き出す
    data.open(20.mebibytes()).stream_to(tokio::io::stdout()).await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
