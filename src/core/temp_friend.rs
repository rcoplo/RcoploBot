use tracing::info;
use crate::core::api::result_entity::{ApiReturnResult, GetStrangerInfoResult};
use crate::core::bot::{ApiResult, Bot, message_handle};
use crate::core::component::event::{FriendMessageEvent, FriendSender, TempFriendMessageEvent, TempFriendSender};
use crate::core::component::message::Message;
use crate::core::friend::Friend;
use crate::service::CONTEXT;

#[derive(Debug,Clone)]
pub struct TempFriend{
    //QQ号
    pub user_id:i64,
    //好友的信息
    pub sender:TempFriendSender,
    //bot
    bot:Bot,
}


impl TempFriend {
    pub fn new(event: &TempFriendMessageEvent, bot:&mut Bot) -> Self{
        let (message,message_list) = message_handle(event.message.clone(), event.raw_message.clone());
        info!("Q::{} > {:?}",&event.user_id,&message);
        Self {
            user_id: event.user_id.clone(),
            sender: event.sender.clone(),
            bot: bot.clone(),
        }
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Result<ApiResult,ApiReturnResult> {
        self.bot.send_private_msg(self.user_id,message).await
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Result<ApiResult,ApiReturnResult> {
        self.bot.send_private_msg_cq(self.user_id,message).await
    }

    pub async fn delete_msg(&mut self,message_id:&i64) -> Result<ApiResult,ApiReturnResult> {
        self.bot.delete_msg(message_id.clone()).await
    }
    pub async fn get_stranger_info(&mut self ) -> Result<GetStrangerInfoResult,ApiReturnResult> {
        self.bot.get_stranger_info(self.user_id).await
    }
    pub fn get_user_avatar(&self) -> String {
        format!("https://q1.qlogo.cn/g?b=qq&nk={}&s=0", self.user_id)
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