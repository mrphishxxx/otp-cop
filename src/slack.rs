use std::io::{Read};

use getopts;

use hyper::{Client};

use rustc_serialize::{json};

use super::{Service, ServiceFactory, ServiceResult, User};


#[derive(RustcDecodable)]
struct SlackUserListResponse {
    ok: bool,
    members: Vec<SlackUser>,
}

#[derive(RustcDecodable)]
struct SlackUser {
    name: String,
    deleted: bool,
    has_2fa: Option<bool>,
    profile: SlackProfile,
    is_owner: Option<bool>,
    is_admin: Option<bool>,
}

#[derive(RustcDecodable)]
struct SlackProfile {
    email: String,
}

pub struct SlackServiceFactory;

impl ServiceFactory for SlackServiceFactory {
    fn add_options(&self, opts: &mut getopts::Options) {
        opts.reqopt(
            "", "slack-token", "Slack token (https://api.slack.com/web#authentication", "token"
        );
    }

    fn create_service(&self, matches: &getopts::Matches) -> Box<Service> {
        return Box::new(SlackService{
            token: matches.opt_str("slack-token").unwrap(),
        });
    }
}

struct SlackService {
    token: String,
}

impl Service for SlackService {
    fn get_users(&self) -> ServiceResult {
        let client = Client::new();

        let mut response = client.get(
            &format!("https://slack.com/api/users.list?token={}", self.token)
        ).send().unwrap();
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        let result = json::decode::<SlackUserListResponse>(&body).unwrap();
        assert!(result.ok);
        let users = result.members.iter().filter(|user| {
            match (user.deleted, user.has_2fa) {
                (true, _) => false,
                (false, None) => true,
                (false, Some(true)) => false,
                (false, Some(false)) => true,
            }
        }).map(|user|
            User{
                name: format!("@{}", user.name.to_string()),
                email: user.profile.email.to_string(),
                details: match (user.is_owner.unwrap(), user.is_admin.unwrap()) {
                    (true, true) => Some("Owner/Admin".to_string()),
                    (true, false) => Some("Owner".to_string()),
                    (false, true) => Some("Admin".to_string()),
                    (false, false) => None
                }
            }
        ).collect();

        return ServiceResult{
            service_name: "Slack".to_string(),
            users: users,
        }
    }
}