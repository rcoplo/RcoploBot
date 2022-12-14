
use std::collections::HashMap;
use once_cell::sync::Lazy;
use rand::Rng;
use serde_json::{Map, Value};
use crate::core::bot::Bot;
use crate::bot::{AiHelp, OsuSbHelp, SetuHelp, SignHelp};
use crate::core::group::Group;
use crate::core::message_chain::MessageChain;
use crate::util::file::{get_image_path, tmp_random_image_path};

pub static BOT_HELP: Lazy<HelpList> = Lazy::new(|| HelpList::default());

pub trait BotHelp {
    fn new() -> Help<'static>;
}

#[derive(Debug,Clone)]
pub struct Help<'a>  {
    //模块名
    pub module_name: String,
    //模块简称
    pub module_name_abbreviation: String,
    //指令集合
    pub module_cmd: HashMap<&'a str,Vec<&'a str>>,
    //默认开关
    pub module_default: bool,
    //帮助详细信息
    pub module_help: Vec<&'a str>,
}
#[derive(Debug)]
pub struct HelpList<'a> {
    pub help: HashMap<String, Help<'a>>,
}
impl Default for HelpList<'_> {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("setu".to_string(), SetuHelp::new());
        map.insert("签到".to_string(), SignHelp::new());
        map.insert("ai".to_string(), AiHelp::new());
        map.insert("osusb".to_string(), OsuSbHelp::new());
        Self {
            help: map
        }
    }
}


pub async fn bot_help_group_handle(group:&mut Group, function: &Map<String, Value>, message_chain: &MessageChain) {
    if message_chain.match_command(&vec!["/help","!help","！help"],&vec![]){
        bot_help_group_image(group, function).await;
    }
}


pub async fn bot_help_group_image(group:&mut Group, function: &Map<String, Value>) {

}