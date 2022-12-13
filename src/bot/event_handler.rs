use std::fmt::Display;
use serde_json::Value;
use crate::bot::{add_group_function, ai_group_module_handle, friend_handle_module, group_change_handle, group_handle_module, open_ai_module_handle, open_group_function, osu_sb_group_module_handle, setu_friend_handle, setu_group_handle, sign_module_handle};
use crate::bot::bot_help::bot_help_group_handle;
use crate::core::bot::BotDataType;
use crate::service::{CONTEXT, GroupFunctionService, SetuService};


pub async fn event_handler(bot_data_type:&mut BotDataType) {
    let group_id = bot_data_type.get_group_id();
    //获取group_id 方便管理群功能
    //在权限里面的功能放这里
    match group_id {
        None => {
        }
        Some(group_id) => {
            //查询当前群的功能
            let function = GroupFunctionService::select_function(&group_id).await;
            match function {
                None => {
                    //如果数据库没有数据则添加
                    add_group_function(&group_id).await;
                }
                Some(fun) => {
                    let string = fun.function_list.unwrap();
                    let result: Value = serde_json::from_str(string.as_str()).unwrap();
                    let function = result.as_object().unwrap();

                    if function.get("groupHelp").unwrap().as_bool() == Some(true) {
                        if let BotDataType::Notice(notice) = bot_data_type {
                            group_change_handle(notice).await;
                        }
                    }
                    if let BotDataType::Group(group,message_chain ) = bot_data_type {
                        bot_help_group_handle(group,function,message_chain).await;
                        open_group_function(group,message_chain).await;
                        if function.get("setu").unwrap().as_bool() == Some(true) {
                            setu_group_handle(group,message_chain).await;
                        }
                        if function.get("签到").unwrap().as_bool() == Some(true) {
                            sign_module_handle(group,message_chain).await;
                        }
                        if function.get("ai").unwrap().as_bool() == Some(true) {
                            ai_group_module_handle(group,message_chain).await;
                            open_ai_module_handle(group,message_chain).await;
                        }
                        if function.get("osusb").unwrap().as_bool() == Some(true) {
                            osu_sb_group_module_handle(group,message_chain).await;
                        }
                    }
                }
            }
        }
    }
    //无权限的普通功能放这里
    match bot_data_type {
        BotDataType::Friend(friend,message_chain) => {
            setu_friend_handle(friend,&message_chain).await;
        }
        BotDataType::Group(group,message_chain) => {

        }
        BotDataType::Bot(bot) => {

        }
        BotDataType::Notice(notice) => {

        }
        BotDataType::Request(request) => {
            friend_handle_module(request).await;
            group_handle_module(request).await;
        }
        BotDataType::Null => {

        }
        BotDataType::TempFriend(temp_friend,message_chain) => {

        }
    }


}

pub fn meow_err<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵...", <M as Into<M>>::into(msg))
}

pub fn meow_ok<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵!", <M as Into<M>>::into(msg))
}

pub fn meow_warn<M: AsRef<str> + Display>(msg: M) -> String {
    format!("{}喵?", <M as Into<M>>::into(msg))
}

