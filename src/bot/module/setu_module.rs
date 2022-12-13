use std::collections::HashMap;
use std::thread;
use std::thread::Thread;

use log::{info, warn};
use regex::{Regex, Replacer};
use serde_yaml::mapping::Index;
use serde_yaml::Value;
use tokio::time;
use crate::core::bot::{ApiResult, Bot};

use crate::core::friend::Friend;
use crate::core::group::Group;

use crate::domain::Setu;
use crate::bot::api::{get_lolicon, get_lolicon_list, get_lolicon_list_tag, get_lolicon_tag};
use crate::bot::{ meow_err};
use crate::bot::bot_help::{BOT_HELP, BotHelp, Help};
use crate::core::api::result_entity::ApiReturnResult;
use crate::core::component::message::{image, text};
use crate::core::message_chain::MessageChain;
use crate::service::{CONTEXT, GroupFunctionService, SetuService};


pub struct SetuHelp;

impl BotHelp for SetuHelp {
    fn new() -> Help<'static> {
        Help {
            module_name: "色图".to_string(),
            module_name_abbreviation: "setu".to_string(),
            module_cmd: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("setu",vec!["/色图", "/瑟图"]),
                ("setu_tag",vec!["/色图[\\s+](.*)", "/瑟图[\\s+](.*)"]),
                ("setu_rand",vec!["/rand 色图", "/rand 瑟图"]),
                ("setu_list",vec![r"(\d)张色图"]),
                ("setu_list_tag",vec!["(\\d)张色图[\\s+](.*)"]),
            ])),
            module_default: false,
            module_help: vec![
                "指令: /色图",
                "参数: {tag}|{tag}",
                "------------------------",
                "指令: {num}张色图",
                "参数: {num}指1~20的数字",
                "     {tag}|{tag}",
                "     {tag}中间以英文 `|` 号间隔",
                "参数和指令中间需有空格",
            ],
        }
    }
}

pub async fn setu_friend_handle(friend: &mut Friend, message_chain: &MessageChain) {
    let setu_help = &BOT_HELP.help.get("setu").unwrap().module_cmd;
    if message_chain.match_command( setu_help.get("setu").unwrap(),&vec![]) {

        setu_friend(friend).await;
    } else if message_chain.match_command( setu_help.get("setu_tag").unwrap(),&vec![]) {

        setu_friend_tag(friend,message_chain).await;
    } else if message_chain.match_command( setu_help.get("setu_rand").unwrap(),&vec![]) {

        rand_setu_friend(friend).await;
    } else if message_chain.match_command( setu_help.get("setu_list").unwrap(),&vec![]) {
        setu_friend_list(friend,message_chain).await;
    } else if message_chain.match_command( setu_help.get("setu_list_tag").unwrap(),&vec![]) {

        setu_friend_list_tag(friend,message_chain).await;
    };
}

pub async fn setu_group_handle(group: &mut Group, message_chain: &MessageChain) {
    let setu_help = &BOT_HELP.help.get("setu").unwrap().module_cmd;
    if message_chain.match_command( setu_help.get("setu").unwrap(),&vec![]) {

        setu_group(group).await;
    } else if message_chain.match_command(setu_help.get("setu_tag").unwrap(),&vec![]) {

        setu_group_tag(group,message_chain).await;
    } else if message_chain.match_command(setu_help.get("setu_rand").unwrap(),&vec![]) {

        // rand_setu_group(&mut group).await;
    } else if message_chain.match_command( setu_help.get("setu_list").unwrap(),&vec![]) {

        setu_group_list(group,message_chain).await;
    } else if message_chain.match_command(setu_help.get("setu_list_tag").unwrap(),&vec![]) {

        setu_group_list_tag(group,message_chain).await;
    };
}

async fn setu_friend(friend: &mut Friend) {
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            rand_setu_friend(friend).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let result = friend.send_private_msg(vec).await;

        }
    }
}

async fn setu_friend_tag(friend: &mut Friend,message_chain:&MessageChain) {
    let mut vec = Vec::new();
    let split: Vec<_> = message_chain.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            rand_setu_friend(friend).await;
        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap())),
                text(format!("pid: {}\n", &setu.pid.unwrap())),
                image(&setu.urls.unwrap()),
            ];
            let result = friend.send_private_msg(vec).await;

        }
    }
}

async fn setu_friend_list(friend: &mut Friend,message_chain:&MessageChain) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(message_chain.message.as_str(), "$last").parse::<i64>().unwrap();

    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap())),
                    text(format!("pid: {}\n", &s.pid.unwrap())),
                    image(&s.urls.unwrap()),
                ];
                let result = friend.send_private_msg(vec).await;

            }
        }
    }
}

async fn setu_friend_list_tag(friend: &mut Friend,message_chain:&MessageChain) {
    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(message_chain.message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let split: Vec<_> = message_chain.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_list_tag(cow, vec).await;
    match lolicon {
        None => {}
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let result = friend.send_private_msg(vec).await;

            }
        }
    }
}

async fn rand_setu_friend(friend: &mut Friend) {
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {

        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let result = friend.send_private_msg(vec).await;

        }
    }
}

async fn setu_group(group: &mut Group) {
    let mut group = group.clone();
    let lolicon = get_lolicon().await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;

        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let result = group.send_group_msg(vec).await;
            match result {
                Ok(data) => {
                    tokio::spawn( async move{
                        delete_msg(&mut group, data.message_id,CONTEXT.config.setu.withdraw_time).await;
                    });
                }
                Err(_) => {
                    group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                }
            }
        }
    }
}

async fn setu_group_list(group: &mut Group,message_chain:&MessageChain) {

    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(message_chain.message.as_str(), "$last").parse::<i64>().unwrap();
    let lolicon = get_lolicon_list(cow).await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;

        }
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", s.pid.unwrap()).as_str()),
                    image(s.urls.unwrap().as_str()),
                ];

                let result = group.send_group_msg(vec).await;
                match result {
                    Ok(data) => {
                        let mut group = group.clone();
                        tokio::spawn( async move{
                            delete_msg(&mut group, data.message_id,CONTEXT.config.setu.withdraw_time).await;
                        });
                    }
                    Err(_) => {
                        group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    }
                }
            }
        }
    }
}

async fn setu_group_list_tag(group: &mut Group, message_chain: &MessageChain) {
    let mut group = group.clone();

    let result = Regex::new(r"(?P<last>\d+)(.*)").unwrap();
    let cow = result.replace(message_chain.message.as_str(), "$last").parse::<i64>().unwrap();
    let mut vec = Vec::new();
    let split: Vec<_> = message_chain.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }

    let lolicon = get_lolicon_list_tag(cow, vec).await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;

        }
        Some(setu) => {
            for s in setu {
                let vec = vec![
                    text(format!("Title: {}\n", &s.title.unwrap()).as_str()),
                    text(format!("pid: {}\n", &s.pid.unwrap()).as_str()),
                    image(&s.urls.unwrap().as_str()),
                ];
                let result = group.send_group_msg(vec).await;
                match result {
                    Ok(data) => {
                        tokio::spawn( async move{
                            delete_msg(&mut group, data.message_id,CONTEXT.config.setu.withdraw_time).await;
                        });
                    }
                    Err(_) => {
                        group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                    }
                }
            }
        }
    }
}

async fn setu_group_tag(group: &mut Group, message_chain: &MessageChain) {
    let mut group = group.clone();
    let mut vec = Vec::new();
    let split: Vec<_> = message_chain.message_list[1].split("|").collect();
    for str in split {
        vec.push(str.to_string())
    }
    let lolicon = get_lolicon_tag(vec).await;
    match lolicon {
        None => {
            let result = group.send_group_msg(vec![text(meow_err("色图获取失败乐"))]).await;

        }
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", &setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", &setu.pid.unwrap()).as_str()),
                image(&setu.urls.unwrap().as_str()),
            ];
            let result = group.send_group_msg(vec).await;
            match result {
                Ok(data) => {
                    tokio::spawn( async move{
                        delete_msg(&mut group, data.message_id,CONTEXT.config.setu.withdraw_time).await;
                    });
                }
                Err(_) => {
                    group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                }
            }
        }
    }
}

async fn rand_setu_group(group: &mut Group) {
    let mut group = group.clone();
    let setu = SetuService::rand_setu().await;
    match setu {
        None => {}
        Some(setu) => {
            let vec = vec![
                text(format!("Title: {}\n", setu.title.unwrap()).as_str()),
                text(format!("pid: {}\n", setu.pid.unwrap()).as_str()),
                image(setu.urls.unwrap().as_str()),
            ];
            let result = group.send_group_msg(vec).await;
            match result {
                Ok(data) => {
                    tokio::spawn( async move{
                        delete_msg(&mut group, data.message_id,CONTEXT.config.setu.withdraw_time).await;
                    });
                }
                Err(_) => {
                    group.send_group_msg(vec![text("这张色图太😍了,我自己看看就好了~")]).await;
                }
            }
        }
    }
}


async fn delete_msg(group: &mut Group,message_id:i64, time: u64) {
    time::sleep(time::Duration::from_secs(time)).await;
    let result = group.delete_msg(message_id).await;
    match result {
        Ok(data) => {
            info!("[Bot] {} - 消息撤回成功!",data.ok);
        }
        Err(err) => {
            warn!("[Bot] {} - 消息撤回失败!",err.wording);
        }
    }
}