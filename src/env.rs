use std::env;

use lazy_static::lazy_static;
use tbot::types::chat::Id;

fn get_env(env: &str) -> String {
    match env::var(env) {
        Ok(var) => var,
        Err(_) => panic!(format!("Environment variable `{}` does not exist", env)),
    }
}

lazy_static! {
    pub static ref GIT_URL: String = get_env("PAVAL_GIT_URL");
    pub static ref GIT_EMAIL: String = get_env("PAVAL_GIT_EMAIL");
    pub static ref GIT_NAME: String = get_env("PAVAL_GIT_NAME");
    pub static ref BOT_TOKEN: String = get_env("PAVAL_BOT_TOKEN");
    pub static ref CHANNEL_ID: Id = Id::from(
        get_env("PAVAL_CHANNEL_ID")
            .parse::<i64>()
            .expect("Invalid PAVAL_CHANNEL_ID")
    );
    pub static ref BOT_URL: String = get_env("WEBHOOK_URL");
    pub static ref PORT: u16 = get_env("PORT").parse().expect("Invalid PORT");
}
