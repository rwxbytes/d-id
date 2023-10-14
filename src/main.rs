use bytes::Bytes;
use http_body_util::{Empty, Full};
use d_id::client::ClientBuilder;
use d_id::endpoints::resources::images::upload_image_by_file;
use d_id::prelude::*;
use d_id::endpoints::video::talks::*;

#[tokio::main]
async fn main() -> Result<()> {
    //let img = upload_image_by_file("img_test_2.jpg").await?;

    //let talk_req_bod = TalkRequestBodyBuilder::with_text_script()?
    //    .source_url(&img.url)?
    //    .input("Read, Write, And Execute Bytes!")?
    //    .build()?;

    //let talk = talk_req_bod.create_talk().await?;

    //let talk = get_talk("tlk_18T34eukR5P0EWdxPMhtt").await?;

    let _del = delete_talk("tlk_18T34eukR5P0EWdxPMhtt").await?;
    let talks = get_talks().await?;


    println!("{:?}", talks);





    Ok(())
}
