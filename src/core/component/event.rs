use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use crate::core::api::result_entity::ApiReturnResult;
use crate::core::bot::{Bot, BotDataType};
use crate::core::component::event;
use crate::core::component::message::Message;


#[derive(Debug)]
pub enum PostType {
    Message(Value),
    Request(Value),
    Notice(Value),
    MetaEvent(Value),
    Null(Value),
}

#[derive(Serialize, Deserialize)]
pub enum NoticeNotifySubType {
    Honor,
    Poke,
    LuckyKing,
}

#[derive(Serialize, Deserialize)]
pub enum SubType {
    Friend,
    Group,
    GroupSelf,
    Normal,
    Anonymous,
    Notice,
}

#[derive(Serialize, Deserialize)]
pub enum MetaEventType {
    Lifecycle,
    Heartbeat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    FriendMessage(FriendMessageEvent),
    TempFriendMessage(TempFriendMessageEvent),
    GroupMessage(GroupMessageEvent),
    GroupFileUpload(GroupFileUploadEvent),
    GroupAdminChange(GroupAdminChangeEvent),
    GroupMemberDecrease(GroupMemberDecreaseEvent),
    GroupMemberIncrease(GroupMemberIncreaseEvent),
    GroupBan(GroupBanEvent),
    FriendAdd(FriendAddEvent),
    GroupMessageRecall(GroupMessageRecallEvent),
    FriendMessageRecall(FriendMessageRecallEvent),
    FriendPoke(FriendPokeEvent),
    GroupPoke(GroupPokeEvent),
    TipsForLuckyKingOfRedPackets(TipsForLuckyKingOfRedPacketsEvent),
    GroupMemberHonorChangePrompt(GroupMemberHonorChangePromptEvent),
    GroupMemberBusinessCardUpdate(GroupMemberBusinessCardUpdateEvent),
    OfflineFileReceived(OfflineFileReceivedEvent),
    AddFriendRequest(AddFriendRequestEvent),
    AddGroupRequest(AddGroupRequestEvent),
    OtherClientOnlineStatusChanges(OtherClientOnlineStatusChangesEvent),
    EssenceMessage(EssenceMessageEvent),
    ApiReturnResult(ApiReturnResult),
}


impl EventType {
    pub fn get_event(v: Value) -> Option<EventType> {
        let post_type = Self::is(v);
        match post_type {
            PostType::Message(json) => {
                let notice_type = json["message_type"].as_str().unwrap_or("null");
                if json["temp_source"].is_null(){
                    match notice_type {
                        "private" =>{
                            return Some(EventType::FriendMessage(serde_json::from_value::<FriendMessageEvent>(json.clone()).unwrap()));
                        }
                        "group" =>{
                            return Some(EventType::GroupMessage(serde_json::from_value::<GroupMessageEvent>(json.clone()).unwrap()));
                        }
                        _ => None
                    }
                }else {
                    return Some(EventType::TempFriendMessage(serde_json::from_value::<TempFriendMessageEvent>(json.clone()).unwrap()));
                }

            }
            PostType::Request(json) => {
                let notice_type = json["request_type"].as_str().unwrap_or("null");
                match notice_type {
                    "friend" =>{
                        return Some(EventType::AddFriendRequest(serde_json::from_value::<AddFriendRequestEvent>(json.clone()).unwrap()));
                    }
                    "group" =>{
                        return Some(EventType::AddGroupRequest(serde_json::from_value::<AddGroupRequestEvent>(json.clone()).unwrap()));
                    }
                    _ => None
                }
            }

            PostType::Notice(json) => {
                let notice_type = json["notice_type"].as_str().unwrap_or("null");
                match notice_type {
                    "group_upload" => {
                        return Some(EventType::GroupFileUpload(serde_json::from_value::<GroupFileUploadEvent>(json.clone()).unwrap()));
                    }
                    "group_admin" => {
                        return Some(EventType::GroupAdminChange(serde_json::from_value::<GroupAdminChangeEvent>(json.clone()).unwrap()));
                    }
                    "group_decrease" => {
                        return Some(EventType::GroupMemberDecrease(serde_json::from_value::<GroupMemberDecreaseEvent>(json.clone()).unwrap()));
                    }
                    "group_increase" => {
                        return Some(EventType::GroupMemberIncrease(serde_json::from_value::<GroupMemberIncreaseEvent>(json.clone()).unwrap()));
                    }
                    "group_ban" => {
                        return Some(EventType::GroupBan(serde_json::from_value::<GroupBanEvent>(json.clone()).unwrap()));
                    }
                    "friend_add" => {
                        return Some(EventType::FriendAdd(serde_json::from_value::<FriendAddEvent>(json.clone()).unwrap()));
                    }
                    "friend_recall" => {
                        return Some(EventType::FriendMessageRecall(serde_json::from_value::<FriendMessageRecallEvent>(json.clone()).unwrap()));
                    }
                    "group_recall" => {
                        return Some(EventType::GroupMessageRecall(serde_json::from_value::<GroupMessageRecallEvent>(json.clone()).unwrap()));
                    }
                    "notify" => {
                        let sub_type = json["sub_type"].as_str().unwrap_or("null");
                        let group_id = json["group_id"].as_i64().unwrap_or(0);
                        return if sub_type.eq("poke") && group_id == 0 {
                            Some(EventType::FriendPoke(serde_json::from_value::<FriendPokeEvent>(json.clone()).unwrap()))
                        } else if sub_type.eq("poke") && group_id != 0 {
                            Some(EventType::GroupPoke(serde_json::from_value::<GroupPokeEvent>(json.clone()).unwrap()))
                        } else if sub_type.eq("lucky_king") {
                            Some(EventType::TipsForLuckyKingOfRedPackets(serde_json::from_value::<TipsForLuckyKingOfRedPacketsEvent>(json.clone()).unwrap()))
                        } else if sub_type.eq("honor") {
                            Some(EventType::GroupMemberHonorChangePrompt(serde_json::from_value::<GroupMemberHonorChangePromptEvent>(json.clone()).unwrap()))
                        } else {
                            None
                        }
                    }
                    "group_card" => {
                        return Some(EventType::GroupMemberBusinessCardUpdate(serde_json::from_value::<GroupMemberBusinessCardUpdateEvent>(json.clone()).unwrap()));
                    }
                    "offline_file" => {
                        return Some(EventType::OfflineFileReceived(serde_json::from_value::<OfflineFileReceivedEvent>(json.clone()).unwrap()));
                    }
                    "client_status" => {
                        return Some(EventType::OtherClientOnlineStatusChanges(serde_json::from_value::<OtherClientOnlineStatusChangesEvent>(json.clone()).unwrap()));
                    }
                    "essence" => {
                        return Some(EventType::EssenceMessage(serde_json::from_value::<EssenceMessageEvent>(json.clone()).unwrap()));
                    }

                    _ => None
                }

            }
            PostType::MetaEvent(json) => {
                //  心跳什么事都不做
                None
            }
            PostType::Null(json) => {
                if let Ok(event) = serde_json::from_value::<ApiReturnResult>(json.clone()) {
                    return Some(EventType::ApiReturnResult(event));
                };
                None
            }
        }
    }
    pub fn is(v: Value) -> PostType {
        let json = v["post_type"].as_str();
        match json {
            None => PostType::Null(v),
            Some(post_type) => {
                match post_type {
                    "message" => PostType::Message(v),
                    "request" => PostType::Request(v),
                    "notice" => PostType::Notice(v),
                    "meta_event" => PostType::MetaEvent(v),
                    _ => PostType::Null(v)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendSender {
    pub age: i32,
    pub nickname: String,
    pub sex: String,
    pub user_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempFriendSender {
    pub age: i32,
    pub group_id: i64,
    pub nickname: String,
    pub sex: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupSender {
    pub age: i32,
    pub area: String,
    pub card: String,
    pub level: String,
    pub nickname: String,
    pub role: String,
    pub sex: String,
    pub title: String,
    pub user_id: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendMessageEvent {
    pub post_type: String,
    pub message_type: String,
    pub time: i64,
    pub self_id: i64,
    pub sub_type: String,
    pub message: Vec<Message>,
    pub raw_message: String,
    pub font: i32,
    pub sender: FriendSender,
    pub message_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TempFriendMessageEvent {
    pub post_type: String,
    pub message_type: String,
    pub time: i64,
    pub self_id: i64,
    pub sub_type: String,
    pub message: Vec<Message>,
    pub raw_message: String,
    pub font: i32,
    pub sender: TempFriendSender,
    pub message_id: i64,
    pub user_id: i64,
    pub temp_source: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageEvent {
    pub post_type: String,
    pub message_type: String,
    pub time: i64,
    pub self_id: i64,
    pub sub_type: String,
    pub sender: GroupSender,
    pub user_id: i64,
    pub anonymous: Value,
    pub font: i32,
    pub group_id: i64,
    pub message: Vec<Message>,
    pub message_seq: i64,
    pub raw_message: String,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupFileUploadEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub file: FileInfo,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub busid: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupAdminChangeEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberDecreaseEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberIncreaseEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupBanEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendAddEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMessageRecallEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub operator_id: i64,
    pub user_id: i64,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendMessageRecallEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
    pub message_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriendPokeEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub sender_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupPokeEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TipsForLuckyKingOfRedPacketsEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub sub_type: String,
    pub user_id: i64,
    pub target_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberHonorChangePromptEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub sub_type: String,
    pub user_id: i64,
    pub honor_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMemberBusinessCardUpdateEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub card_new: String,
    pub card_old: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflineFileReceivedEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub user_id: i64,
    pub file: OfflineFileInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfflineFileInfo {
    pub name: String,
    pub size: i64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddFriendRequestEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub request_type: String,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddGroupRequestEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub sub_type: String,
    pub request_type: String,
    pub group_id: i64,
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherClientOnlineStatusChangesEvent {
    pub post_type: String,
    pub notice_type: String,
    pub client: Device,
    pub online: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub app_id: i64,
    pub device_name: String,
    pub device_kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EssenceMessageEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub notice_type: String,
    pub sub_type: String,
    pub sender_id: i64,
    pub operator_id: i64,
    pub message_id: i64,
}