// #![feature(core)]
#![feature(custom_derive, plugin)]
extern crate curl;
extern crate rustc_serialize;
extern crate ini;
extern crate regex;

use std::str;
use std::string;
use std::collections::BTreeMap;
use std::collections::HashMap;
use regex::Regex;
mod mod_util;
mod mod_user;
mod mod_channel;
mod mod_history;

pub fn main() {
  let api_conf = mod_util::util::get_slack_api_conf("slack.conf");
  
  // get user list
  let users : mod_user::user::Users = mod_user::user::get_users(&(api_conf.token));
  
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
  let mut channel_id = mod_channel::channel::get_channel_id(&(api_conf.token), &(api_conf.channel)).unwrap();
  let messages : mod_history::history::Messages = mod_history::history::get_messages(&(api_conf.token), &(channel_id));
  let mut messages_it = messages.messages.into_iter();
  let re = Regex::new(r"<@(U[a-zA-Z0-9]+)").unwrap();
  loop {
    if let Some(message) = messages_it.next() {
      let mut text_str = match str::from_utf8(message.text.as_bytes()) {	  
        Ok(e) => e,
        Err(e) => panic!("Invalid UTF-8 sequence"),
      };
      let mut final_str = text_str.to_string();
      for cap in re.captures_iter(text_str) {
        let user_id = cap.at(1).unwrap_or("").to_string();
        if !id_name_mapping.contains_key(&user_id) {
          continue;
        }
        final_str = final_str.replace(cap.at(1).unwrap_or(""), id_name_mapping.get(&user_id).unwrap());
      }
      println!("{}", final_str);
    } else {
      break;
    }
  }
  // println!("data: {:?}", data);
}