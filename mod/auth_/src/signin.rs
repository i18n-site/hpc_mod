use aok::{Result, OK};
use req_::{SetHeader, SET_COOKIE};

#[iat::captcha]
pub async fn mail(address: &str, password: &str, set_header: &SetHeader) -> Result<()> {
  set_header.push(SET_COOKIE, "x=111");
  OK
}
