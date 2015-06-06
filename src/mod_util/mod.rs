// misc
pub mod util {
  use ini::Ini;
  use curl::http;
  use std::str;
  pub struct slack_api_info
  {
    pub token: String,
    pub channel: String
  }
  
  /*
  fn print_type_of<T>(_: &T) -> () {
    let type_name =
      unsafe {
        std::intrinsics::type_name::<T>()
      };
    println!("{}", type_name);
  }
  */

  pub fn get_slack_api_conf<'a>(ini_path: &'a str) -> slack_api_info {
    let mut conf = Ini::load_from_file(ini_path).unwrap();
    let sec = conf.section("Slack");
    let token = sec.get("token").unwrap();
    let channel = sec.get("channel").unwrap();
    slack_api_info {
      token: token.clone(),
      channel: channel.clone()
    }
  }
  
  pub fn get_http_content<'a>(url: &'a str) -> String {
    let resp = http::handle().get(url).exec().unwrap();
    let content_str = match str::from_utf8(resp.get_body()) {
      Ok(e) => e,
      Err(e) => panic!("Invalid UTF-8 sequence"),
    };
    content_str.to_string()
  }
}