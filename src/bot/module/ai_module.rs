use std::collections::HashMap;
use log::info;
use once_cell::sync::Lazy;
use rand::Rng;
use regex::RegexSet;
use url::form_urlencoded::parse;
use crate::core::group::Group;
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::service::{CONTEXT, SignGroupUsersService};
use crate::util::file::get_data_path;
use std::default::Default;
use std::fmt::Formatter;
use rbatis::rbdc::datetime::FastDateTime;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use crate::core::component::cq::Cq;
use crate::core::component::message;
use crate::core::component::message::{json, reply, text};
use crate::core::message_chain::MessageChain;
use crate::domain::SignGroupUsers;
use crate::util::http_utils::post_json;

pub static AI: Lazy<Vec<Ai>> = Lazy::new(|| Ai::new());
pub static  OPENAI_URL:&str = "https://api.openai.com/v1/completions";
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ai {
    pub r#type: String,
    pub data: Vec<String>,
}

pub struct AiHelp;

impl BotHelp for AiHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "ai".to_string(),
            module_name_abbreviation: "ai".to_string(),
            module_cmd: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("ai",vec![
                    "{bot_name}{msg}",
                    "{msg}{bot_name}",
                    "{at_bot}{msg}",
                ]),
                ("open_ai",vec![
                    "/ai[\\s+](.*)",
                ]),
            ])),
            module_default: true,
            module_help: vec![
                "{bot_name}{msg}",
                "{msg}{bot_name}",
                "{at_bot}{msg}",
            ],
        }
    }
}

pub async fn ai_group_module_handle(group: &mut Group, message_chain: &MessageChain) {
    let ai_help = &BOT_HELP.help.get("ai").unwrap().module_cmd;
    if message_chain.match_command(ai_help.get("ai").unwrap(),&vec!["摸+","好感度"]) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                 group.send_group_msg(vec![text("喵... 你还没有签到喵...,要签了到才能互动喵!,摸摸小白吧~~")]).await;
            }
            Some(data) => {
                let time = FastDateTime::now();
                let data_time = &data.checkin_count_last.unwrap();
                if (&time.get_day() == &data_time.get_day()) && (&time.get_mon() == &data_time.get_mon()) {
                    let ai = &AI.to_vec();
                    let name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
                    let regex_set = RegexSet::new(vec![format!(r"[{}]", &message_chain.msg())]).unwrap();
                    for data in ai {
                        if regex_set.is_match(data.r#type.as_str()) {
                            let i = data.data.len();
                            let i1 = rand::thread_rng().gen_range(0..i);

                             group.send_group_msg(vec![text(data.data.get(i1).unwrap())]).await;

                            return;
                        }
                    }
                } else {
                     group.send_group_msg(vec![text("喵... 你还没有签到喵...,要签了到才能互动喵!,摸摸小白吧~~")]).await;

                }
            }
        }
    }
}

impl Ai {
    pub fn new() -> Vec<Self> {
        let data = include_str!("../../../resources/data/data.json");
        let ai: Vec<Ai> = serde_yaml::from_str(data).expect("[Bot] load config file fail");
        ai
    }
}
/// OpenAI module
pub async fn open_ai_module_handle(group: &mut Group, message_chain: &MessageChain){
    let ai_help = &BOT_HELP.help.get("ai").unwrap().module_cmd;
    if message_chain.match_command(ai_help.get("open_ai").unwrap(),&vec![]) {
        let string = &CONTEXT.config.api.openai_api_key;
        let msg = message_chain.message_list[1].replace("\\", "");
        let nickname = group.sender.nickname.clone();
        let user = group.sender.user_id.clone();
        let name_id = format!("{}{}", nickname, user);
        let json = json!({
            "model": "text-curie-001",
            "prompt": msg,
            "max_tokens": 100,
            "temperature": 1,
            "user": name_id
        });
        let mut map = HeaderMap::new();
        map.insert("Content-Type",HeaderValue::from_str("application/json").unwrap());
        map.insert("Authorization",HeaderValue::from_str(format!("Bearer {}",string).as_str()).unwrap());
        let option = post_json(OPENAI_URL, map, json).await;
        match option {
            None => {}
            Some(data) => {
                info!("{}",&data);
                if data["error"].is_null() {
                    let response = serde_json::from_value::<OpenAiResponse>(data).unwrap();
                    let mut string1 = String::new();
                    for choices in response.choices {
                        string1.push_str(choices.text.as_str());
                    }
                    let string2 = string1.replace("\n\n", "");
                    group.send_group_msg(vec![reply(&message_chain.message_id),text(string2)]).await;
                }else {
                    let str = data["error"]["message"].as_str().unwrap().replace("\n\n", "");
                    group.send_group_msg(vec![reply(&message_chain.message_id),text(str)]).await;
                }

            }
        }
    }

}
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct OpenAiResponse{
    pub id:String,
    pub object:String,
    pub created:i64,
    pub model:String,
    pub choices: Vec<Choices>,
    pub usage: Usage,
}
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct Choices {
    pub text:String,
    pub index:i64,
    pub finish_reason:String,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct Usage {
    pub prompt_tokens:i32,
    pub completion_tokens:i32,
    pub total_tokens:i32,
}