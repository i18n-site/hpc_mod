use aok::Result;
use click_captcha::ICO_LI;
use rand::{Rng, SeedableRng, rngs::StdRng};
use xbin::concat;
use xkv::{
  R,
  fred::{interfaces::KeysInterface, types::Expiration},
};

const SIZE: u32 = 350;

pub const R_CAPTCHA: &[u8] = b"captcha:";

pub struct Captcha {
  pub id: Box<[u8]>,
  pub img: Box<[u8]>,
  pub tip: Box<[u8]>,
}

pub async fn captcha() -> Result<Captcha> {
  let (img, meta) = click_captcha::webp(SIZE, SIZE, ICO_LI)?;

  let mut rng = StdRng::from_entropy();

  let id: [u8; 16] = rng.r#gen();

  R.set::<(), _, _>(
    concat!(R_CAPTCHA, id),
    &pc::e(meta.pos_li)?[..],
    Some(Expiration::EX(60)),
    None,
    false,
  )
  .await?;

  let tip = meta
    .ico_li
    .into_iter()
    .map(|i| ICO_LI[i])
    .collect::<Vec<&'static str>>()
    .join("|")
    .as_bytes()
    .into();

  Ok(Captcha {
    id: id.into(),
    img,
    tip,
  })
}
