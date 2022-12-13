use log::info;
use tracing::event;

use crate::core::api::result_entity::{ApiReturnResult, GetStrangerInfoResult};
use crate::core::bot::{Bot, ApiResult};
use crate::core::component::event::{EssenceMessageEvent, FriendAddEvent, FriendMessageRecallEvent, FriendPokeEvent, GroupAdminChangeEvent, GroupBanEvent, GroupFileUploadEvent, GroupMemberBusinessCardUpdateEvent, GroupMemberDecreaseEvent, GroupMemberHonorChangePromptEvent, GroupMemberIncreaseEvent, GroupMessageRecallEvent, GroupPokeEvent, OfflineFileReceivedEvent, OtherClientOnlineStatusChangesEvent, TipsForLuckyKingOfRedPacketsEvent};
use crate::core::component::message::Message;
use crate::service::CONTEXT;

#[derive(Debug, Clone)]
pub enum Notice {
    GroupFileUpload{
        event: GroupFileUploadEvent,
        bot:Bot,
    },
    GroupAdminChange{
        event: GroupAdminChangeEvent,
        bot:Bot,
    },
    GroupMemberDecrease{
        event: GroupMemberDecreaseEvent,
        bot:Bot,
    },
    GroupMemberIncrease{
        event: GroupMemberIncreaseEvent,
        bot:Bot,
    },
    GroupBan{
        event: GroupBanEvent,
        bot:Bot,
    },
    FriendAdd{
        event: FriendAddEvent,
        bot:Bot,
    },
    GroupMessageRecall{
        event: GroupMessageRecallEvent,
        bot:Bot,
    },
    FriendMessageRecall{
        event: FriendMessageRecallEvent,
        bot:Bot,
    },
    FriendPoke{
        event: FriendPokeEvent,
        bot:Bot,
    },
    GroupPoke{
        event: GroupPokeEvent,
        bot:Bot,
    },
    TipsForLuckyKingOfRedPackets{
        event: TipsForLuckyKingOfRedPacketsEvent,
        bot:Bot,
    },
    GroupMemberHonorChangePrompt{
        event: GroupMemberHonorChangePromptEvent,
        bot:Bot,
    },
    GroupMemberBusinessCardUpdate{
        event: GroupMemberBusinessCardUpdateEvent,
        bot:Bot,
    },
    OfflineFileReceived{
        event: OfflineFileReceivedEvent,
        bot:Bot,
    },
    OtherClientOnlineStatusChanges{
        event: OtherClientOnlineStatusChangesEvent,
        bot:Bot,
    },
    EssenceMessage{
        event: EssenceMessageEvent,
        bot:Bot,
    },

}
impl Notice {
    pub async fn send_group_msg(&mut self, message: Vec<Message>) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Notice::GroupFileUpload { event,bot} => {
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
            Notice::GroupAdminChange { event,bot } => {
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
            Notice::GroupMemberDecrease { event,bot} => {
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
            Notice::GroupMemberIncrease { event,bot } => {
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
            Notice::GroupBan { event,bot } => {
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
            Notice::GroupMessageRecall { event,bot } => {
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
            Notice::GroupPoke { event,bot } => {
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
            Notice::TipsForLuckyKingOfRedPackets { event,bot } => {
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
            Notice::GroupMemberHonorChangePrompt { event,bot } => {
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
            Notice::GroupMemberBusinessCardUpdate { event,bot } => {
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
    pub async fn send_group_msg_cq(&mut self, message: String) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Notice::GroupFileUpload { event,bot} => {
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
            Notice::GroupAdminChange { event,bot } => {
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
            Notice::GroupMemberDecrease { event,bot} => {
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
            Notice::GroupMemberIncrease { event,bot } => {
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
            Notice::GroupBan { event,bot } => {
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

            Notice::GroupMessageRecall { event,bot } => {
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

            Notice::GroupPoke { event,bot } => {
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
            Notice::TipsForLuckyKingOfRedPackets { event,bot } => {
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
            Notice::GroupMemberHonorChangePrompt { event,bot } => {
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
            Notice::GroupMemberBusinessCardUpdate { event,bot } => {
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
    pub async fn send_private_msg(&mut self, message: Vec<Message>) -> Result<ApiResult,Option<ApiReturnResult>> {
        match self {
            Notice::GroupFileUpload { event,bot} => {
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
            Notice::GroupAdminChange { event,bot } => {
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
            Notice::GroupMemberDecrease { event,bot} => {
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
            Notice::GroupMemberIncrease { event,bot } => {
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
            Notice::GroupBan { event,bot } => {
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

            Notice::GroupMessageRecall { event,bot } => {
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

            Notice::GroupPoke { event,bot } => {
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
            Notice::TipsForLuckyKingOfRedPackets { event,bot } => {
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
            Notice::GroupMemberHonorChangePrompt { event,bot } => {
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
            Notice::GroupMemberBusinessCardUpdate { event,bot } => {
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

            Notice::FriendAdd { event,bot } => {
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
            Notice::FriendMessageRecall { event,bot } => {
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
            Notice::FriendPoke { event,bot} => {
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
            Notice::OfflineFileReceived { event,bot } => {
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
            Notice::GroupFileUpload { event,bot} => {
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
            Notice::GroupAdminChange { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberDecrease { event,bot} => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberIncrease { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupBan { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMessageRecall { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupPoke { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::TipsForLuckyKingOfRedPackets { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberHonorChangePrompt { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberBusinessCardUpdate { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendAdd { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendMessageRecall { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendPoke { event,bot} => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            Notice::OfflineFileReceived { event,bot } => {
                let result = bot.send_private_msg_cq(event.user_id, message).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) =>{
                        Err(Some(err))
                    }
                }
            }
            _ => {
                Err(None)
            }
        }
    }
    pub async fn get_stranger_info(&mut self) -> Result<GetStrangerInfoResult, Option<ApiReturnResult>> {
        match self {
            Notice::GroupFileUpload { event,bot} => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupAdminChange { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberDecrease { event,bot} => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberIncrease { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupBan { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMessageRecall { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupPoke { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::TipsForLuckyKingOfRedPackets { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberHonorChangePrompt { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberBusinessCardUpdate { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }

            Notice::FriendAdd { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendMessageRecall { event,bot} => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendPoke { event,bot} => {
                let result = bot.get_stranger_info(event.user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::OfflineFileReceived { event,bot } => {
                let result = bot.get_stranger_info(event.user_id).await;
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
    }pub async fn get_stranger_info_user(&mut self,user_id:i64) -> Result<GetStrangerInfoResult, Option<ApiReturnResult>> {
        match self {
            Notice::GroupFileUpload { bot, ..} => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupAdminChange { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberDecrease { bot, ..} => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberIncrease { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupBan { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMessageRecall { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupPoke { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::TipsForLuckyKingOfRedPackets {bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberHonorChangePrompt { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::GroupMemberBusinessCardUpdate { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }

            Notice::FriendAdd { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendMessageRecall { bot, ..} => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::FriendPoke { bot, ..} => {
                let result = bot.get_stranger_info(user_id).await;
                match result {
                    Ok(data) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(Some(err))
                    }
                }
            }
            Notice::OfflineFileReceived { bot, .. } => {
                let result = bot.get_stranger_info(user_id).await;
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
            Notice::GroupFileUpload { event,bot} => {
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
            Notice::GroupAdminChange {event,bot} => {
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
            Notice::GroupMemberDecrease {event,bot} => {
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
            Notice::GroupMemberIncrease {event,bot} => {
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
            Notice::GroupBan {event,bot} => {
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

            Notice::GroupMessageRecall {event,bot} => {
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
            Notice::GroupPoke {event,bot} => {
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
            Notice::TipsForLuckyKingOfRedPackets {event,bot} => {
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
            Notice::GroupMemberHonorChangePrompt {event,bot} => {
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
            Notice::GroupMemberBusinessCardUpdate {event,bot} => {
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
            Notice::GroupFileUpload {event, .. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupAdminChange { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupMemberDecrease { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupMemberIncrease { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupBan {event, .. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::FriendAdd {event, .. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupMessageRecall { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::FriendMessageRecall { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::FriendPoke {event, .. } => {for bot_id in super_admin {
                if bot_id == event.user_id {
                    return true;
                }
            }
                false

            }
            Notice::GroupPoke { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::TipsForLuckyKingOfRedPackets {event, .. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupMemberHonorChangePrompt { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::GroupMemberBusinessCardUpdate {event, .. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            Notice::OfflineFileReceived { event,.. } => {
                for bot_id in super_admin {
                    if bot_id == event.user_id {
                        return true;
                    }
                }
                false
            }
            _ => false
        }
    }
}



