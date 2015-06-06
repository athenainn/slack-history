// #![feature(core)]
#![feature(custom_derive, plugin)]
extern crate curl;
extern crate rustc_serialize;
extern crate ini;

use std::str;
use std::string;
use std::collections::BTreeMap;
use std::collections::HashMap;

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

// user
pub mod user {
  extern crate rustc_serialize;
  use util;
  use rustc_serialize::json;
  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Profile {
    // pub first_name: String,
    // pub last_name: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub email: String,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String
  }

  #[derive(RustcDecodable, RustcEncodable)]
  pub struct User {
    pub id: String,
    pub name: String,
    pub deleted: bool,
    // pub status: String,
    pub color: String,
    pub real_name: String,
    pub tz: String,
    pub tz_label: String,
    pub tz_offset: i32,
    pub profile: Profile,
    pub is_admin: bool,
    pub is_owner: bool,
    pub is_primary_owner: bool,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub is_bot: bool,
    pub has_files: bool,
    pub has_2fa: bool
  }

  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Users {
    pub ok: bool,
    pub members: Vec<User>
  }
  
  pub fn get_users(token: &String) -> Users {
    let user_list_api = format!("https://slack.com/api/users.list?token={}&pretty=1", token);
    let user_list_content = util::get_http_content(user_list_api.trim());
    let users : Users = json::decode(&user_list_content.trim()).unwrap();
    users
  }
}

pub mod channel {
  extern crate rustc_serialize;
  use util;
  use rustc_serialize::json;
  
  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Channel {
    pub id: String,
    pub name: String,
    pub created: i32,
    pub creator: String,
    pub num_members: i32
  }
  
  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Channels {
    pub ok: bool,
    pub channels: Vec<Channel>
  }
  
  pub fn get_channels(token: &String) -> Channels {
    let channel_list_api = format!("https://slack.com/api/channels.list?token={}&pretty=1", token);
    let channel_list_content = util::get_http_content(channel_list_api.trim());
    let channels : Channels = json::decode(&channel_list_content.trim()).unwrap();
    channels
  }
  
  pub fn get_channel_id(token: &String, channel_name: &String) -> Option<String> {
    let channels : self::Channels = self::get_channels(token);
    let mut channel_id = None;
    let mut channels_it = channels.channels.into_iter();
    loop {
      if let Some(channel) = channels_it.next() {
        if channel.name[..] == channel_name[..] {
          channel_id = Some(channel.id);
          break;
        }
      } else {
        break;
      }
    }
    channel_id
  }
}

pub mod history {
  extern crate rustc_serialize;
  use util;
  use rustc_serialize::json;
  
  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Message {
    pub user: String,
    pub text: String,
  }
  
  #[derive(RustcDecodable, RustcEncodable)]
  pub struct Messages {
    pub ok: bool,
    pub messages: Vec<Message>
  }
  
  pub fn get_messages(token: &String, channel: &String) -> Messages {
    let history_list_api = format!("https://slack.com/api/channels.history?token={}&channel={}", token, channel);
    let history_list_content = util::get_http_content(history_list_api.trim());
    let messages : Messages = json::decode(&history_list_content.trim()).unwrap();
    messages
  }
}

pub fn main() {
  let api_conf = util::get_slack_api_conf("slack.conf");
  
  // get user list
  let users : user::Users = user::get_users(&(api_conf.token));
  
  // let's build a id->name mapping.
  let mut id_name_mapping = HashMap::new();
  let mut users_it = users.members.into_iter();
  loop {
    if let Some(member) = users_it.next() {
      id_name_mapping.insert(member.id, member.name);
    } else {
      break;
    }
  }

  // get history
  let mut channel_id = channel::get_channel_id(&(api_conf.token), &(api_conf.channel)).unwrap();
  let messages : history::Messages = history::get_messages(&(api_conf.token), &(channel_id));
  let mut messages_it = messages.messages.into_iter();
  loop {
    if let Some(message) = messages_it.next() {
      let text_str = match str::from_utf8(message.text.as_bytes()) {
        Ok(e) => e,
        Err(e) => panic!("Invalid UTF-8 sequence"),
      };
      println!("{}", text_str);
    } else {
      break;
    }
  }
  // println!("data: {:?}", data);
}