use std::sync::Arc;
use log::info;
use regex::{Error, RegexSet, Replacer};
use crate::core::api::result_entity::{ApiReturnResult, GetGroupInfoResult, GetStrangerInfoResult};
use crate::core::bot::{Bot, message_handle, ApiResult};
use crate::core::component::event::{ GroupMessageEvent, GroupSender};
use crate::core::component::message::Message;
use crate::service::CONTEXT;


#[derive(Debug,Clone)]
pub struct Group {
    /// 群号
    pub group_id: i64,
    ///群成员
    pub user_id: i64,
    ///群成员的信息
    pub sender: GroupSender,
    ///bot
    bot: Bot,
}

impl Group {
    pub fn new(event: &GroupMessageEvent, bot:&mut Bot) -> Self {
        Self {
            group_id: event.group_id.clone(),
            user_id:event.user_id.clone(),
            sender: event.sender.clone(),
            bot: bot.clone(),
        }

    }
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Result<ApiResult,ApiReturnResult> {
        self.bot.send_group_msg(self.group_id, message).await
    }

    pub async fn send_group_msg_cq(&mut self, message: String) -> Result<ApiResult,ApiReturnResult> {
        self.bot.send_group_msg_cq(self.group_id, message).await
    }

    pub async fn send_group_forward_msg(&mut self, message: Vec<Message>) -> Result<ApiResult,ApiReturnResult> {
        self.bot.send_group_forward_msg(self.group_id, message).await
    }

    pub async fn delete_msg(&mut self,message_id:i64) -> Result<ApiResult,ApiReturnResult> {
        self.bot.delete_msg(message_id).await
    }

    pub async fn get_stranger_info(&mut self ) -> Result<GetStrangerInfoResult,ApiReturnResult> {
        self.bot.get_stranger_info(self.user_id).await
    }

    pub fn get_group_avatar(&self) -> String {
        format!("https://p.qlogo.cn/gh/{0}/{0}/0/", self.group_id)
    }

    ///是管理员/群主/超级管理员 才响应
    pub async fn is_admin(&self) -> bool{
        let mut bot = self.bot.clone();
        let super_admin = CONTEXT.bot_config.super_administrator.clone();
        let group_list = bot.get_group_member_list(self.group_id).await;
        match group_list {
            Ok(list) => {
                for group_member_info in list {
                    if group_member_info.role.eq(&"owner".to_string()) || group_member_info.role.eq(&"admin".to_string()){
                        return true;
                    }
                }
                for bot_id in super_admin {
                    if bot_id == self.user_id{
                        return true;
                    }
                }
                false
            }
            Err(_) => false
        }
    }
    ///是超级管理员 才响应
    pub fn is_super_admin(&self) -> bool{
        let super_admin = CONTEXT.bot_config.super_administrator.clone();
        for bot_id in super_admin {
            if bot_id == self.user_id {
                return true;
            }
        }
        false
    }
}

