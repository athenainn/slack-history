pub mod channel {
  extern crate rustc_serialize;
  use mod_util;
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
    let channel_list_content = mod_util::util::get_http_content(channel_list_api.trim());
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