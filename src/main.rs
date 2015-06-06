#![feature(core)]
#![feature(custom_derive, plugin)]
extern crate curl;
extern crate rustc_serialize;
extern crate ini;
use rustc_serialize::json;

use curl::http;
use std::str;
use std::string;
use ini::Ini;
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
struct Profile {
  // first_name: String,
  // last_name: String,
  real_name: String,
  real_name_normalized: String,
  email: String,
  image_24: String,
  image_32: String,
  image_48: String,
  image_72: String,
  image_192: String
}

#[derive(RustcDecodable, RustcEncodable)]
struct User {
  id: String,
  name: String,
  deleted: bool,
  // status: String,
  color: String,
  real_name: String,
  tz: String,
  tz_label: String,
  tz_offset: i32,
  profile: Profile,
  is_admin: bool,
  is_owner: bool,
  is_primary_owner: bool,
  is_restricted: bool,
  is_ultra_restricted: bool,
  is_bot: bool,
  has_files: bool,
  has_2fa: bool
}

#[derive(RustcDecodable, RustcEncodable)]
struct Users {
  ok: bool,
  members: Vec<User>
}

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
  let mut user_list_api = format!("https://slack.com/api/users.list?token={}&pretty=1", api_conf.token);
  let content = get_http_content(user_list_api.trim());
  // println!("{}", content.trim());
  let users : Users = json::decode(&content.trim()).unwrap();
  // println!("data: {:?}", data);
}