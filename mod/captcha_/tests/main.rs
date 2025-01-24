use aok::{OK, Result};
use captcha_::{R_CAPTCHA, captcha};
use click_captcha::PosLi;
use xbin::concat;
use xkv::{R, fred::interfaces::KeysInterface};
// use tracing::info;

#[tokio::test]
async fn test() -> Result<()> {
  xboot::init().await?;

  let captcha = captcha().await?;
  let pos_li: PosLi = pc::d(R.get::<Vec<u8>, _>(concat!(R_CAPTCHA, captcha.id)).await?)?;

  let mut click_x_y = vec![];
  for i in pos_li.iter() {
    let size = i.size / 2;
    click_x_y.push(i.x + size);
    click_x_y.push(i.y + size);
  }
  assert!(!(captcha_verify::run(&captcha.id, &[0, 0, 0, 0, 0, 0]).await?));
  assert!(captcha_verify::run(&captcha.id, &click_x_y).await?);
  assert!(!(captcha_verify::run(&captcha.id, &click_x_y).await?));
  OK
}

// #[test]
// fn test() -> Result<()> {
//   info!("{}", 123456);
//   OK
// }
