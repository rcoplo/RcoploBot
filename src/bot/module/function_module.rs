use std::collections::HashMap;
use log::{error, info};
use once_cell::sync::Lazy;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::Error;
use serde_json::{json, Value};
use crate::core::bot::Bot;
use crate::core::friend::Friend;
use crate::core::group::Group;

use crate::domain::GroupFunction;
use crate::bot::bot_help::BOT_HELP;
use crate::bot::{ meow_err, meow_ok, meow_warn};
use crate::core::component::message::text;
use crate::core::message_chain::MessageChain;
use crate::pool;
use crate::service::{CONTEXT, GroupFunctionService};


pub async fn add_group_function(group_id: &i64) {
    let map_help = &BOT_HELP.help;
    let mut map = HashMap::new();
    for (abb, help) in map_help.iter() {
        map.insert(abb.clone(), help.module_default);
    }
    let value = Value::from_iter(map);
    let function1 = GroupFunction {
        id: 0,
        group_id: Some(group_id.clone()),
        function_list: Some(value.to_string()),
        modify_time: Some(FastDateTime::now()),
    };
    GroupFunctionService::insert_function(function1).await;
}


pub async fn open_group_function(group: &mut Group, message_chain: &MessageChain) {
    let vec1 = CONTEXT.bot_config.super_administrator.clone();
    if group.is_admin().await {
        if message_chain.match_command( &vec!["开启[\\s]+\\w+"],&vec![]) {
            if GroupFunctionService::open_function(&group.group_id, &message_chain.msg()).await {
                group.send_group_msg(vec![text(meow_ok(format!("开启 {} 功能成功", &message_chain.msg())))]).await;
            } else {
                group.send_group_msg(vec![text(meow_warn(format!("开启 {} 功能失败,可能该功能并不存在", &message_chain.msg())))]).await;
            };
        } else if message_chain.match_command(  &vec!["关闭[\\s]+\\w+"],&vec![]) {
            if GroupFunctionService::close_function(&group.group_id, &message_chain.msg()).await {
                group.send_group_msg(vec![text(meow_ok(format!("关闭 {} 功能成功", &message_chain.msg())))]).await;
            } else {
                group.send_group_msg(vec![text(meow_warn(format!("关闭 {} 功能失败,可能该功能并不存在", &message_chain.msg())))]).await;
            };
        }
    }
}

pub async fn friend_function_handle(friend:&mut Friend) {

}