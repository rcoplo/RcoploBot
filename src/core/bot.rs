use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use futures_util::lock::Mutex;
use log::info;
use once_cell::sync::Lazy;
use serde_json::{ json, Map, Value};
use serde::{Deserialize, Serialize};
use tracing::{event, warn};

use crate::core::api::api::*;
use crate::core::api::base_api::BaseApi;
use crate::core::api::result_entity::*;
use crate::core::component::message::{Message, message_type_handle};
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::message_chain::MessageChain;
use crate::core::notice::Notice;
use crate::core::request::Request;
use crate::core::temp_friend::TempFriend;

#[derive(Debug)]
pub enum BotDataType {
    Friend(Friend,MessageChain),
    TempFriend(TempFriend,MessageChain),
    Group(Group,MessageChain),
    Bot(Bot),
    Notice(Notice),
    Request(Request),
    Null
}

#[derive(Debug, Clone)]
pub struct Bot {
    pub bot_id: i64,
    pub api_sender: mpsc::Sender<ApiResult>,
    pub resp_promises: Arc<Mutex<HashMap<String, oneshot::Sender<ApiResult>>>>,
}

#[derive(Debug,Clone,serde::Deserialize,serde::Serialize)]
pub struct ApiResult {
    pub echo: String,
    pub ok: bool,
    pub data: Option<Value>,
    pub return_data:Result<(),ApiReturnResult>,
    pub message_id: i64,
}

fn base_api_to_json<P: Serialize>(base_api: BaseApi<P>) -> Value {
    serde_json::to_value(&base_api).unwrap()
}


impl Bot {
    async fn send_and_wait<P: Serialize>(&mut self, api: BaseApi<P>) -> Result<ApiResult,ApiReturnResult> {
        let echo = uuid::Uuid::new_v4().to_string();
        let api = api;
        let data = BaseApi {
            echo: echo.clone(),
            ..api
        };
        let data = base_api_to_json(data);
        let frame = ApiResult {
            echo: echo.clone(),
            ok: true,
            data: Some(data),
            return_data: Ok(()),
            message_id: 0,
        };
        // 发送API请求
        let api_sender = mpsc::Sender::clone(&self.api_sender);
        api_sender.send(frame).await.unwrap();
        // 等待API响应
        let (resp_sender, mut resp_receiver) = oneshot::channel();
        self.resp_promises.lock().await.insert(echo.clone(), resp_sender);
        let api_resp_frame = resp_receiver.await.unwrap();
        match api_resp_frame.return_data {
            Ok(_) => {
                info!("[Bot] API response > {} > {}",&api_resp_frame.ok,&api_resp_frame.message_id);
                Ok(api_resp_frame)
            }
            Err(err) => {
                warn!("[Bot] API response > {} > {}",&err.msg,&err.wording);
                Err(err)
            }
        }
    }
    async fn send_and_wait_echo<P: Serialize>(&mut self,echo:uuid::Uuid, api: BaseApi<P>) -> Result<ApiResult,ApiReturnResult> {
        let echo = echo.to_string();
        let api = api;
        let data = BaseApi {
            echo: echo.clone(),
            ..api
        };
        let data = base_api_to_json(data);
        let frame = ApiResult {
            echo: echo.clone(),
            ok: true,
            data: Some(data),
            return_data: Ok(()),
            message_id: 0,
        };
        // 发送API请求
        let api_sender = mpsc::Sender::clone(&self.api_sender);
        api_sender.send(frame).await;

        // 等待API响应
        let (resp_sender, mut resp_receiver) = oneshot::channel();
        self.resp_promises.lock().await.insert(echo.clone(), resp_sender);
        let api_resp_frame = resp_receiver.await.unwrap();
        match api_resp_frame.return_data {
            Ok(_) => {
                info!("[Bot] API response > {} > {}",&api_resp_frame.ok,&api_resp_frame.message_id);
                Ok(api_resp_frame)
            }
            Err(err) => {
                warn!("[Bot] API response > {} > {}",&err.msg,&err.wording);
                Err(err)
            }
        }
    }

    pub async fn send_private_msg(&mut self, user_id: i64, message: Vec<Message>, ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendPrivateMsg {
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_private_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn send_private_msg_cq(&mut self, user_id: i64, message: String) -> Result<ApiResult,ApiReturnResult> {
        let re = SendPrivateMsgCq {
            user_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_private_msg_cq().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn send_group_msg(&mut self, group_id: i64, message: Vec<Message>, ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupMsg {
            group_id,
            message,
            auto_escape: false,
        };
        let api = re.send_group_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_group_msg_cq(&mut self, group_id: i64, message: String) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupMsgCq {
            group_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_group_msg_cq().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_group_forward_msg(&mut self, group_id: i64, message: Vec<Message>) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupForwardMsg {
            group_id,
            message,
        };
        let api = re.send_group_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_msg(&mut self, message_type: &str, group_id: i64, user_id: i64, message: Message) -> Result<ApiResult,ApiReturnResult> {
        let re = SendMsg {
            message_type: message_type.to_string(),
            group_id,
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_private_msg_echo(&mut self, echo:uuid::Uuid, user_id: i64, message: Vec<Message>, ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendPrivateMsg {
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_private_msg().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn send_private_msg_cq_echo(&mut self,echo:uuid::Uuid, user_id: i64, message: String) -> Result<ApiResult,ApiReturnResult> {
        let re = SendPrivateMsgCq {
            user_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_private_msg_cq().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn send_group_msg_echo(&mut self, echo:uuid::Uuid, group_id: i64, message: Vec<Message>, ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupMsg {
            group_id,
            message,
            auto_escape: false,
        };
        let api = re.send_group_msg().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_group_msg_cq_echo(&mut self,echo:uuid::Uuid, group_id: i64, message: String) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupMsgCq {
            group_id,
            message: message.to_string(),
            auto_escape: false,
        };
        let api = re.send_group_msg_cq().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_group_forward_msg_echo(&mut self, echo:uuid::Uuid, group_id: i64, message: Vec<Message>) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupForwardMsg {
            group_id,
            message,
        };
        let api = re.send_group_forward_msg().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_msg_echo(&mut self, echo:uuid::Uuid, message_type: &str, group_id: i64, user_id: i64, message: Message) -> Result<ApiResult,ApiReturnResult> {
        let re = SendMsg {
            message_type: message_type.to_string(),
            group_id,
            user_id,
            message,
            auto_escape: false,
        };
        let api = re.send_msg().await.unwrap();
        let resp = self.send_and_wait_echo(echo,api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn delete_msg(&mut self, message_id: i64) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteMsg {
            message_id,
        };
        let api = re.delete_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_msg(&mut self, message_id: i64) -> Result<GetMsgResult,ApiReturnResult> {
        let re = GetMsg {
            message_id,
        };
        let api = re.get_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetMsgResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => Err(err)
        }
    }
    pub async fn get_forward_msg(&mut self, message_id: i64) -> Result<GetForwardMsgResult,ApiReturnResult> {
        let re = GetForwardMsg {
            message_id,
        };
        let api = re.get_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetForwardMsgResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => Err(err)
        }
    }
    pub async fn get_image(&mut self, file: &str) -> Result<GetImageResult,ApiReturnResult> {
        let re = GetImage {
            file: file.to_string(),
        };
        let api = re.get_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetImageResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => Err(err)
        }
    }

    pub async fn can_send_image(&mut self) -> Result<ApiResult,ApiReturnResult> {
        let api = CanSendImage.can_send_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn mark_msg_as_read(&mut self, message_id: i64) -> Result<ApiResult,ApiReturnResult> {
        let re = MarkMsgAsRead {
            message_id,
        };
        let api = re.mark_msg_as_read().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_kick(
        &mut self,
        group_id: i64,
        user_id: i64,
        reject_add_request: bool,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupKick {
            group_id,
            user_id,
            reject_add_request,
        };
        let api = re.set_group_kick().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_ban(
        &mut self,
        group_id: i64,
        user_id: i64,
        duration: Value,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupBan {
            group_id,
            user_id,
            duration,
        };
        let api = re.set_group_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_anonymous_ban(
        &mut self,
        group_id: i64,
        flag: &str,
        duration: Value,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupAnonymousBan {
            group_id,
            flag: flag.to_string(),
            duration,
        };
        let api = re.set_group_anonymous_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_whole_ban(
        &mut self,
        group_id: i64,
        enable: bool,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupWholeBan {
            group_id,
            enable,
        };
        let api = re.set_group_whole_ban().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_admin(
        &mut self,
        group_id: i64,
        user_id: i64,
        enable: bool,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupAdmin {
            group_id,
            user_id,
            enable,
        };
        let api = re.set_group_admin().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_card(
        &mut self,
        group_id: i64,
        user_id: i64,
        card: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupCard {
            group_id,
            user_id,
            card: card.to_string(),
        };
        let api = re.set_group_card().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_name(
        &mut self,
        group_id: i64,
        group_name: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupName {
            group_id,
            group_name: group_name.to_string(),
        };
        let api = re.set_group_name().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_leave(
        &mut self,
        group_id: i64,
        is_dismiss: bool,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupLeave {
            group_id,
            is_dismiss,
        };
        let api = re.set_group_leave().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_special_title(
        &mut self,
        group_id: i64,
        user_id: i64,
        special_title: bool,
        duration: Value,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupSpecialTitle {
            group_id,
            user_id,
            special_title,
            duration,
        };
        let api = re.set_group_special_title().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_group_sign(
        &mut self,
        group_id: i64,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupSign {
            group_id,
        };
        let api = re.send_group_sign().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_friend_add_request(
        &mut self,
        flag: &str,
        approve: bool,
        remark: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetFriendAddRequest {
            flag: flag.to_string(),
            approve,
            remark: remark.to_string(),
        };
        let api = re.set_friend_add_request().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_add_request(
        &mut self,
        flag: &str,
        sub_type: &str,
        approve: bool,
        reason: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupAddRequest {
            flag: flag.to_string(),
            sub_type: sub_type.to_string(),
            approve,
            reason: reason.to_string(),
        };
        let api = re.set_group_add_request().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_login_info(
        &mut self,
    ) -> Result<GetLoginInfoResult,ApiReturnResult> {
        let api = GetLoginInfo.get_login_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetLoginInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {
                Err(err)
            }
        }

    }
    pub async fn set_qq_profile(
        &mut self,
        nickname: &str,
        company: &str,
        email: &str,
        college: &str,
        personal_note: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetQqProfile {
            nickname: nickname.to_string(),
            company: company.to_string(),
            email: email.to_string(),
            college: college.to_string(),
            personal_note: personal_note.to_string(),
        };
        let api = re.set_qq_profile().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_stranger_info(
        &mut self,
        user_id: i64,
    ) -> Result<GetStrangerInfoResult,ApiReturnResult> {
        let re = GetStrangerInfo {
            user_id,
            no_cache: false,
        };
        let api = re.get_stranger_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetStrangerInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_friend_list(
        &mut self,
    ) -> Result<Vec<GetFriendListResult>,ApiReturnResult> {
        let api = GetFriendList.get_friend_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<Vec<GetFriendListResult>>(data.data.unwrap()).unwrap())
            }
            Err(err) =>  Err(err)
        }
    }
    pub async fn get_unidirectional_friend_list(
        &mut self,
    ) -> Result<Vec<GetUnidirectionalFriendListResult>,ApiReturnResult> {
        let api = GetUnidirectionalFriendList.get_unidirectional_friend_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<Vec<GetUnidirectionalFriendListResult>>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn delete_friend(
        &mut self,
        friend_id: i64,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteFriend {
            friend_id,
        };
        let api = re.delete_friend().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_group_info(
        &mut self,
        group_id: i64,
    ) -> Result<GetGroupInfoResult,ApiReturnResult> {
        let re = GetGroupInfo {
            group_id,
            no_cache: false,
        };
        let api = re.get_group_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_list(
        &mut self,
    ) -> Result<Vec<GetGroupInfoResult>,ApiReturnResult> {
        let api = GetGroupList.get_group_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<Vec<GetGroupInfoResult>>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_member_info(
        &mut self,
        group_id: i64,
        user_id: i64,
    ) -> Result<GetGroupMemberInfoResult,ApiReturnResult> {
        let re = GetGroupMemberInfo {
            group_id,
            user_id,
            no_cache: false,
        };
        let api = re.get_group_member_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupMemberInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_member_list(
        &mut self,
        group_id: i64,
    ) -> Result<Vec<GetGroupMemberInfoResult>,ApiReturnResult> {
        let re = GetGroupMemberList {
            group_id,
            no_cache: false,
        };
        let api = re.get_group_member_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<Vec<GetGroupMemberInfoResult>>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_honor_info(
        &mut self,
        group_id: i64,
        r#type: &str,
    ) -> Result<GetGroupHonorInfoResult,ApiReturnResult> {
        let re = GetGroupHonorInfo {
            group_id,
            r#type: r#type.to_string(),
        };
        let api = re.get_group_honor_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupHonorInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn can_send_record(
        &mut self,
    ) -> Result<ApiResult,ApiReturnResult> {
        let api = CanSendRecord.can_send_record().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_version_info(
        &mut self,
    ) -> Result<GetVersionInfoResult,ApiReturnResult> {
        let api = GetVersionInfo.get_version_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetVersionInfoResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn set_restart(
        &mut self,
        delay: Value,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetRestart {
            delay,
        };
        let api = re.set_restart().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn set_group_portrait(
        &mut self,
        group_id: i64,
        file: &str,
        cache: i32,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetGroupPortrait {
            group_id,
            file: file.to_string(),
            cache,
        };
        let api = re.set_group_portrait().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_word_slices(
        &mut self,
        content: &str,
    ) -> Result<GetWordSlicesResult,ApiReturnResult> {
        let re = GetWordSlices {
            content: content.to_string(),
        };
        let api = re.get_word_slices().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetWordSlicesResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn ocr_image(
        &mut self,
        image: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = OcrImage {
            image: image.to_string(),
        };
        let api = re.ocr_image().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_group_system_msg(
        &mut self,
    ) -> Result<GetGroupSystemMsgResult,ApiReturnResult> {
        let api = GetGroupSystemMsg.get_group_system_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupSystemMsgResult>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn upload_private_file_local(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = UploadPrivateFile {
            user_id,
            file: file.to_string(),
            name: name.to_string(),
        };
        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn upload_private_file_header(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
        headers: Vec<String>,
    ) -> Result<ApiResult,ApiReturnResult> {
        let file = self.download_file(file, headers).await;
        let url = match file {
            Ok(data) => {
                let value = data.data.unwrap();
                value["file"].to_string()
            }
            Err(_) => "".to_string()
        };
        let re = UploadPrivateFile {
            user_id,
            file: url,
            name: name.to_string(),
        };

        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn upload_private_file(
        &mut self,
        user_id: i64,
        file: &str,
        name: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let file = self.download_file(file, vec![]).await;
        let url = match file {
            Ok(data) => {
                let value = data.data.unwrap();
                value["file"].to_string()
            }
            Err(_) => {"".to_string()}
        };
        let re = UploadPrivateFile {
            user_id,
            file: url,
            name: name.to_string(),
        };

        let api = re.upload_private_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn upload_group_file_local(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = UploadGroupFile {
            group_id,
            file: file.to_string(),
            name: name.to_string(),
            folder: folder.to_string(),
        };
        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn upload_group_file_header(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
        headers: Vec<String>,
    ) -> Result<ApiResult,ApiReturnResult> {
        let file = self.download_file(file, headers).await;
        let url = match file {
            Ok(data) => {
                let value = data.data.unwrap();
                value["file"].to_string()
            }
            Err(_) => {"".to_string()}
        };
        let re = UploadGroupFile {
            group_id,
            file: url,
            name: name.to_string(),
            folder: folder.to_string(),
        };

        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn upload_group_file(
        &mut self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let file = self.download_file(file, vec![]).await;
        let url = match file {
            Ok(data) => {
                let value = data.data.unwrap();
                value["file"].to_string()
            }
            Err(_) => {"".to_string()}
        };
        let re = UploadGroupFile {
            group_id,
            file: url,
            name: name.to_string(),
            folder: folder.to_string(),
        };

        let api = re.upload_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn download_file(
        &mut self,
        url: &str,
        headers: Vec<String>,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DownloadFile {
            url: url.to_string(),
            thread_count: 8,
            headers,
        };
        let api = re.download_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_group_file_system_info(
        &mut self,
        group_id: i64,
    ) -> Result<GetGroupFileSystemInfoRequest,ApiReturnResult> {
        let re = GetGroupFileSystemInfo {
            group_id,
        };
        let api = re.get_group_file_system_info().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupFileSystemInfoRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_root_files(
        &mut self,
        group_id: i64,
    ) -> Result<GetGroupRootFilesRequest,ApiReturnResult> {
        let re = GetGroupRootFiles {
            group_id,
        };
        let api = re.get_group_root_files().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupRootFilesRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_files_by_folder(
        &mut self,
        group_id: i64,
        folder_id: &str,
    ) -> Result<GetGroupFilesByFolderRequest,ApiReturnResult> {
        let re = GetGroupFilesByFolder {
            group_id,
            folder_id: folder_id.to_string(),
        };
        let api = re.get_group_files_by_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupFilesByFolderRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn create_group_file_folder(
        &mut self,
        group_id: i64,
        name: &str,
        parent_id: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = CreateGroupFileFolder {
            group_id,
            name: name.to_string(),
            parent_id: parent_id.to_string(),
        };
        let api = re.create_group_file_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn delete_group_folder(
        &mut self,
        group_id: i64,
        folder_id: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteGroupFolder {
            group_id,
            folder_id: folder_id.to_string(),
        };
        let api = re.delete_group_folder().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn delete_group_file(
        &mut self,
        group_id: i64,
        file_id: &str,
        busid: i32,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteGroupFile {
            group_id,
            file_id: file_id.to_string(),
            busid,
        };
        let api = re.delete_group_file().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_group_file_url(
        &mut self,
        group_id: i64,
        file_id: &str,
        busid: i32,
    ) -> Result<GetGroupFileUrlRequest,ApiReturnResult> {
        let re = GetGroupFileUrl {
            group_id,
            file_id: file_id.to_string(),
            busid,
        };
        let api = re.get_group_file_url().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupFileUrlRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_status(
        &mut self,
    ) -> Result<GetStatusRequest,ApiReturnResult> {
        let api = GetStatus.get_status().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetStatusRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_at_all_remain(
        &mut self,
        group_id: i64,
    ) -> Result<GetGroupAtAllRemainRequest,ApiReturnResult> {
        let re = GetGroupAtAllRemain {
            group_id,
        };
        let api = re.get_group_at_all_remain().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupAtAllRemainRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn handle_quick_operation(
        &mut self,
        context: Value,
        operation: Value,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = HandleQuickOperation {
            context,
            operation,
        };
        let api = re.handle_quick_operation().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn _send_group_notice(
        &mut self,
        group_id: i64,
        content: &str,
        image: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendGroupNotice {
            group_id,
            content: content.to_string(),
            image: image.to_string(),
        };
        let api = re._send_group_notice().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn _get_group_notice(
        &mut self,
        group_id: i64,
    ) -> Result<GetGroupNoticeRequest,ApiReturnResult> {
        let re = GetGroupNotice {
            group_id,
        };
        let api = re._get_group_notice().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupNoticeRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn reload_event_filter(
        &mut self,
        file: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = ReloadEventFilter {
            file: file.to_string(),
        };
        let api = re.reload_event_filter().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_online_clients(
        &mut self,
    ) -> Result<GetOnlineClientsRequest,ApiReturnResult> {
        let re = GetOnlineClients {
            no_cache: false
        };
        let api = re.get_online_clients().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetOnlineClientsRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn get_group_msg_history(
        &mut self,
        group_id: i64,
        message_seq: i64,
    ) -> Result<GetGroupMsgHistoryRequest,ApiReturnResult> {
        let re = GetGroupMsgHistory {
            message_seq,
            group_id,
        };
        let api = re.get_group_msg_history().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetGroupMsgHistoryRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn set_essence_msg(
        &mut self,
        message_id: i64,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetEssenceMsg {
            message_id,
        };
        let api = re.set_essence_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn delete_essence_msg(
        &mut self,
        message_id: i64,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteEssenceMsg {
            message_id,
        };
        let api = re.delete_essence_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn get_essence_msg_list(
        &mut self,
        group_id: i64,
    ) -> Result<Vec<GetEssenceMsgListRequest>,ApiReturnResult> {
        let re = GetEssenceMsgList {
            group_id,
        };
        let api = re.get_essence_msg_list().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<Vec<GetEssenceMsgListRequest>>(data.data.unwrap()).unwrap())
            }
            Err(err) => {Err(err)}
        }
    }
    pub async fn check_url_safely(
        &mut self,
        url: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = CheckUrlSafely {
            url: url.to_string(),
        };
        let api = re.check_url_safely().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn _get_model_show(
        &mut self,
        model: &str,
    ) -> Result<GetModelShowRequest,ApiReturnResult> {
        let re = GetModelShow {
            model: model.to_string(),
        };
        let api = re._get_model_show().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(serde_json::from_value::<GetModelShowRequest>(data.data.unwrap()).unwrap())
            }
            Err(err) => Err(err)
        }

    }
    pub async fn _set_model_show(
        &mut self,
        model: &str,
        model_show: &str,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SetModelShow {
            model: model.to_string(),
            model_show: model_show.to_string(),
        };
        let api = re._set_model_show().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn delete_unidirectional_friend(
        &mut self,
        user_id: i64,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = DeleteUnidirectionalFriend {
            user_id,
        };
        let api = re.delete_unidirectional_friend().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    pub async fn send_private_forward_msg(
        &mut self,
        user_id: i64,
        message: Message,
    ) -> Result<ApiResult,ApiReturnResult> {
        let re = SendPrivateForwardMsg {
            user_id,
            message,
        };
        let api = re.send_private_forward_msg().await.unwrap();
        let resp = self.send_and_wait(api).await;
        match resp {
            Ok(data) => {
                Ok(data)
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
pub fn message_handle(message:Vec<Message>, raw_message:String) -> (String, Vec<String>){
    let mut vec = vec![];
    //将string[] message 消息格式化
    let message = message_type_handle(message);
    //将raw_message 以空格分组
    let msg_list:Vec<_> = raw_message.split_whitespace().collect();
    for msg in msg_list {
        vec.push(msg.to_string());
    }
    (message,vec)
}

impl BotDataType{
    pub fn new() -> BotDataType{
        BotDataType::Null
    }

    pub fn get_group_id(&self) -> Option<i64> {
        match self {
            BotDataType::Friend( ..) => None,
            BotDataType::Group(group,..) => Some(group.group_id),
            BotDataType::Bot(..) => None,
            BotDataType::Notice(notice,..) => {
                match notice {
                    Notice::GroupFileUpload { event, .. } => Some(event.group_id),
                    Notice::GroupAdminChange {  event,.. } => Some(event.group_id),
                    Notice::GroupMemberDecrease {  event,.. } => Some(event.group_id),
                    Notice::GroupMemberIncrease {  event,.. } => Some(event.group_id),
                    Notice::GroupBan {  event,.. } => Some(event.group_id),
                    Notice::GroupMessageRecall {  event,.. } => Some(event.group_id),
                    Notice::GroupPoke {  event,.. } => Some(event.group_id),
                    Notice::TipsForLuckyKingOfRedPackets {  event,.. } => Some(event.group_id),
                    Notice::GroupMemberHonorChangePrompt {  event,.. } => Some(event.group_id),
                    Notice::GroupMemberBusinessCardUpdate { event, .. } => Some(event.group_id),
                    _ => None
                }
            },
            BotDataType::Request(req,..) => {
                match req {
                    Request::AddFriendRequest { .. } => None,
                    Request::AddGroupRequest { event,.. } => Some(event.group_id),
                }
            },
            BotDataType::Null => None,
            BotDataType::TempFriend(temp,..) => Some(temp.sender.group_id)
        }
    }
}