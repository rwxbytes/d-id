use d_id::prelude::*;
use http_body_util::Empty;
use d_id::endpoints::resources::images::{delete_image, upload_image_by_file};

#[tokio::main]
async fn main() -> Result<()> {
    //let img = upload_image_by_file("img_test.jpg").await?;
    //println!("{:?}", img.id);
    let _resp = delete_image("img_kglq8EjZzQPv5ZPTVOWwJ").await?;


    Ok(())
}
