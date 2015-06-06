pub mod history {
  extern crate rustc_serialize;
  use mod_util;
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
    let history_list_content = mod_util::util::get_http_content(history_list_api.trim());
    let messages : Messages = json::decode(&history_list_content.trim()).unwrap();
    messages
  }
}