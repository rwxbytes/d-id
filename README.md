A lib crate for interacting with [D-ID's](https://docs.d-id.com/reference/get-started) Api


| API            | Support |
|----------------|---------|
| Live Streaming | ❌       |

## ⚙️ Requirements

- Set API key as environment variable `D_ID_API_KEY`

## 🗣️ Usage

## Talks

- Create talking head videos from just text or audio

```rust
use d_id::{get_talk, TalkRequestBodyBuilder, upload_image_by_file, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let image = upload_image_by_file("img_test.jpg").await?;

    let req= TalkRequestBodyBuilder::with_text_script()
        .source_url(&image.url)?
        .input("Hello, world!")?
        .build()?;

    let new_talk = req.create_talk().await?;

    let talk = get_talk(&new_talk.id).await?;

    println!("{:#?}", talk);

    Ok(())

}
```

