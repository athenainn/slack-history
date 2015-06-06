pub mod user {
  extern crate rustc_serialize;
  use mod_util;
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
    let user_list_content = mod_util::util::get_http_content(user_list_api.trim());
    let users : Users = json::decode(&user_list_content.trim()).unwrap();
    users
  }
}