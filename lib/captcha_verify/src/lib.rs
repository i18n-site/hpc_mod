use aok::Result;
use click_captcha::{PosLi, verify};
use xbin::concat;
use xkv::{R, fred::interfaces::KeysInterface};

pub async fn run(id: &[u8], click_x_y: &[u32]) -> Result<bool> {
  let key = concat!(b"captcha:", id);
  if let Some(pos_li) = R.get::<Option<Vec<u8>>, _>(key).await? {
    let pos_li: PosLi = pc::d(pos_li)?;
    let r = verify(pos_li, click_x_y);
    if r {
      R.del::<(), _>(key).await?;
    }
    return Ok(r);
  }
  Ok(false)
}
