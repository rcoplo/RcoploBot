use std::collections::HashMap;
use log::info;
use rand::Rng;
use rbatis::rbdc::datetime::FastDateTime;
use crate::core::group::Group;
use crate::domain::SignGroupUsers;
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::core::component::message::{at, text};
use crate::core::message_chain::MessageChain;
use crate::service::{CONTEXT, SignGroupUsersService};


pub struct SignHelp;

impl BotHelp for SignHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "Sign 简易的签到功能".to_string(),
            module_name_abbreviation: "签到".to_string(),
            module_cmd: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("momo", vec![
                    "{bot_name}摸+",
                    "摸+{bot_name}",
                    "{at_bot}摸+"]),
                ("good_feeling",
                 vec!["{bot_name}好感度",
                      "好感度{bot_name}",
                      "{at_bot}好感度"]),
            ])),
            module_default: true,
            module_help: vec![
                "[摸+]{bot_name}",
                "{bot_name}[摸+]",
            ],
        }
    }
}

pub async fn sign_module_handle(group: &mut Group, message_chain: &MessageChain) {
    let sign_help = BOT_HELP.help.get("签到").unwrap();
    if message_chain.match_command(sign_help.module_cmd.get("momo").unwrap(), &vec![]) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                let sign = SignGroupUsersService::insert_sign(&group.user_id, &group.group_id).await;
                if sign {
                    let i = rand::thread_rng().gen_range(0..101);
                    group.send_group_msg(vec![
                        at(&group.user_id),
                        text(" 喵喵~~签到成功了喵！\n"),
                        text(format!("心情值: {}", i).as_str()),
                    ]).await;
                }
            }
            Some(data) => {
                let time = FastDateTime::now();
                let data_time = &data.checkin_count_last.unwrap();
                if (&time.get_day() == &data_time.get_day()) && (&time.get_mon() == &data_time.get_mon()) {
                    group.send_group_msg(vec![
                        at(&group.user_id),
                        text(" 喵？今天你已经签到过了喵！"),
                    ]).await;
                } else {
                    let sign1 = SignGroupUsersService::sign(&group.user_id, &group.group_id).await;
                    if sign1 {
                        let i = rand::thread_rng().gen_range(0..101);
                        group.send_group_msg(vec![
                            at(&group.user_id),
                            text(" 喵喵~~签到成功了喵！\n"),
                            text(format!("心情值: {}", i).as_str()),
                        ]).await;
                    }
                }
            }
        }
    } else if message_chain.match_command(sign_help.module_cmd.get("good_feeling").unwrap(), &vec![]) {
        let sign2 = SignGroupUsersService::select_is_sign(&group.user_id, &group.group_id).await;
        match sign2 {
            None => {
                group.send_group_msg(vec![
                    at(&group.user_id),
                    text(" 喵... 咱没有你的好感喵...,先摸摸小白吧~~ "),
                ]).await;
            }
            Some(data) => {
                group.send_group_msg(vec![
                    at(&group.user_id),
                    text("咱对你的好感度为: "),
                    text(format!("{} 喵...", data.impression.unwrap()).as_str()),
                ]).await;
            }
        }
    }
}




