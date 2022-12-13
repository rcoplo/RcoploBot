use futures_util::future::err;
use log::info;
use crate::core::api::result_entity::ApiReturnResult;
use crate::core::bot::{Bot, ApiResult};
use crate::core::component::event::{AddFriendRequestEvent, AddGroupRequestEvent};
use crate::core::component::message::Message;
use crate::service::CONTEXT;


#[derive(Debug,Clone)]
pub enum  Request {
    AddFriendRequest{
        event: AddFriendRequestEvent,
        bot:Bot,
    },
    AddGroupRequest{
        sub_type: GroupAddSubType,
        event: AddGroupRequestEvent,
        bot:Bot,
    }

}

impl Request {
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddGroupRequest { event,bot,..  } => {
                let result = bot.send_group_msg(event.group_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }

    pub async fn send_group_msg_cq(&mut self, message: String) ->Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddGroupRequest { event,bot ,.. } => {
                let result = bot.send_group_msg_cq(event.group_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }

    pub async fn send_private_msg(&mut self, message: Vec<Message>, ) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddFriendRequest { event,bot  } => {
                let result = bot.send_private_msg(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }

    pub async fn send_private_msg_cq(&mut self, message: String) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddFriendRequest { event,bot  } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }
    pub async fn set_group_add_request(&mut self,approve:bool, reason: &str,) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddGroupRequest { event,bot,sub_type } => {
                let result =  match sub_type {
                    GroupAddSubType::Add => {
                        bot.set_group_add_request(event.flag.as_str(),"add",approve,reason).await
                    }
                    GroupAddSubType::Invite => {
                        bot.set_group_add_request(event.flag.as_str(),"invite",approve,reason).await
                    }
                };
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }
    pub async fn set_friend_add_request(&mut self,approve:bool, reason: &str,) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Request::AddFriendRequest { event,bot } => {
                let result =  bot.set_friend_add_request(event.flag.as_str(),approve,reason).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            _ => Err(None)
        }
    }
    ///是管理员/群主/超级管理员 才响应
    pub async fn is_admin(&mut self) -> bool{
        let super_admin = CONTEXT.bot_config.super_administrator.clone();
        match self {
            Request::AddGroupRequest { event,bot,.. } => {
                let group_list = bot.get_group_member_list(event.group_id).await;
                match group_list {
                    Ok(list) => {
                        for group_member_info in list {
                            if group_member_info.role.eq(&"owner".to_string()) || group_member_info.role.eq(&"admin".to_string()){
                                return true;
                            }

                        }
                        for bot_id in super_admin {
                            if bot_id == event.user_id{
                                return true;
                            }
                        }
                        false
                    }
                    Err(_) => false
                }
            }
            _ => false
        }


    }
    ///是超级管理员 才响应
    pub fn is_super_admin(&self) -> bool{
        let super_admin = CONTEXT.bot_config.super_administrator.clone();
        match self {
            Request::AddFriendRequest { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Request::AddGroupRequest { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
        }
    }
}
#[derive(Debug,Clone)]
pub enum GroupAddSubType {
    Add,
    Invite,

}
impl GroupAddSubType {
    pub fn new(sub_type: &String) -> Self {
        match sub_type.as_str() {
            "add" => GroupAddSubType::Add,
            "invite" => GroupAddSubType::Invite,
            _ => GroupAddSubType::Add
        }
    }
}