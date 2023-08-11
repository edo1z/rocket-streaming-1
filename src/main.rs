#[macro_use]
extern crate rocket;

use multer::{parse_boundary, Multipart};
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use tokio_util::io::ReaderStream;

#[post("/", data = "<data>")]
async fn index(content_type: &ContentType, data: Data<'_>) -> String {
    let boundary = parse_boundary(content_type.to_string()).ok();
    if boundary.is_none() {
        println!("No boundary found");
        return "No boundary found".to_string();
    }

    let stream = ReaderStream::new(data.open(20.mebibytes()));
    let mut multipart = Multipart::new(stream, boundary.unwrap());
    while let Some(mut field) = multipart.next_field().await.ok().flatten() {
        let name = field.name();
        let file_name = field.file_name();
        if file_name.is_none() {
            println!("Name: {:?}", name);
            let value = field.text().await.ok();
            println!("value: {:?}", value);
        } else {
            let content_type = field.content_type().unwrap().to_string();
            println!("Name: {:?}, File Name: {:?}, Content-Type: {:?}", name, file_name, content_type);
            let mut first_chunk = true;
            while let Some(chunk) = field.chunk().await.ok().flatten() {
                if first_chunk {
                    first_chunk = false;
                    let kind = infer::get(&chunk);
                    if kind.is_none() {
                        println!("no kind");
                    }
                    let mime_type = kind.unwrap().mime_type();
                    println!("mime_type: {:?}", mime_type);
                    if content_type != mime_type {
                        println!("invalid file type");
                        return "invalid file type".to_string();
                    }
                }
            }
        }
    }
    "Ok".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
