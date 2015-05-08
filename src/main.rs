#![feature(core)]
extern crate curl;
extern crate ini;

use curl::http;
use std::str;
use std::string;
use ini::Ini;

struct slack_api_info
{
  token: String,
  channel: String  
}

fn print_type_of<T>(_: &T) -> () {
  let type_name =
    unsafe {
      std::intrinsics::type_name::<T>()
    };
  println!("{}", type_name);
}

fn get_slack_api_conf<'a>(ini_path: &'a str) -> slack_api_info {
  let mut conf = Ini::load_from_file(ini_path).unwrap();
  let sec = conf.section("Slack");
  let token = sec.get("token").unwrap();
  let channel = sec.get("channel").unwrap();
  slack_api_info {
    token: token.clone(),
    channel: channel.clone()
  }
}

fn get_http_content<'a>(url: &'a str) -> String {
  let resp = http::handle().get(url).exec().unwrap();
  let content_str = match str::from_utf8(resp.get_body()) {
    Ok(e) => e,
    Err(e) => panic!("Invalid UTF-8 sequence"),
  };
  content_str.to_string()
}

pub fn main() {
  let api_conf = get_slack_api_conf("slack.conf");
  let user_list_api = format!("https://slack.com/api/users.list?token={}&pretty=1", api_conf.token);
  let content = get_http_content(user_list_api.trim());
  println!("result is {}", content);
}
