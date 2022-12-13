mod ws;
mod wsr;

use tracing::info;
use crate::core::network::ws::WsClient;
use wsr::WsServer;
use crate::core::bot::{Bot, BotDataType};
use crate::core::component::event::EventType;
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::message_chain::MessageChain;
use crate::core::notice::Notice;
use crate::core::request::{GroupAddSubType, Request};
use crate::core::temp_friend::TempFriend;
use crate::service::CONTEXT;

pub struct Ws;
impl Ws {
    pub async fn run()  {
        if CONTEXT.bot_config.r#type.eq(&Some("ws".to_string())) {
            WsClient::run().await;
        }else if CONTEXT.bot_config.r#type.eq(&Some("ws-reverse".to_string())){
            WsServer::run().await;
        }
    }
}


pub struct DefaultHandler;
impl DefaultHandler   {
    pub async fn handle(event: EventType, bot: &mut Bot) -> BotDataType {

        match event {
            EventType::FriendMessage(event) => {
                info!("Q::{} > {:?}",&event.user_id,&event.raw_message);
                BotDataType::Friend(Friend::new(&event, bot),MessageChain::init(&event.message_id,&event.message,&event.raw_message,bot))
            },
            EventType::TempFriendMessage(event) => {
                info!("Q::{} > {:?}",&event.user_id,&event.raw_message);
                BotDataType::TempFriend(TempFriend::new(&event,bot),MessageChain::init(&event.message_id,&event.message,&event.raw_message,bot))
            }
            EventType::GroupMessage(event) => {
                info!("G::{} > Q::{} > {:?}",&event.group_id,&event.user_id,&event.raw_message);
                BotDataType::Group(Group::new(&event,bot),MessageChain::init(&event.message_id,&event.message,&event.raw_message,bot))
            },
            EventType::GroupFileUpload(event) => {
                info!("G::{} > Q::{} > Event: GroupFileUpload ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupFileUpload {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupAdminChange(event) => {
                info!("G::{} > Q::{} > Event: GroupAdminChange ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupAdminChange {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupMemberDecrease(event) =>  {
                info!("G::{} > Q::{} > Event: GroupMemberDecrease ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupMemberDecrease {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupMemberIncrease(event) => {
                info!("G::{} > Q::{} > Event: GroupMemberIncrease ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupMemberIncrease {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupBan(event) => {
                info!("G::{} > Q::{} > Event: GroupBan ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupBan {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::FriendAdd(event) => {
                info!("Q::{} > Event: FriendAdd ",&event.user_id);
                BotDataType::Notice(Notice::FriendAdd {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupMessageRecall(event) => {
                info!("G::{} > Q::{} > Event: GroupMessageRecall ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupMessageRecall {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::FriendMessageRecall(event) => {
                info!("Q::{} > Event: FriendMessageRecall ", &event.user_id);
                BotDataType::Notice(Notice::FriendMessageRecall {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::FriendPoke(event) => {
                info!("Q::{} > Event: FriendPoke ", &event.user_id);
                BotDataType::Notice(Notice::FriendPoke {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupPoke(event) => {
                info!("G::{} > Q::{} > Event: GroupPoke ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupPoke {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::TipsForLuckyKingOfRedPackets(event) => {
                info!("G::{} > Q::{} > Event: TipsForLuckyKingOfRedPackets ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::TipsForLuckyKingOfRedPackets {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupMemberHonorChangePrompt(event) => {
                info!("G::{} > Q::{} > Event: GroupMemberHonorChangePrompt ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupMemberHonorChangePrompt {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::GroupMemberBusinessCardUpdate(event) => {
                info!("G::{} > Q::{} > Event: GroupMemberBusinessCardUpdate ", &event.group_id,&event.user_id);
                BotDataType::Notice(Notice::GroupMemberBusinessCardUpdate {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::OfflineFileReceived(event) => {
                info!("Q::{} > Event: OfflineFileReceived ",&event.user_id);
                BotDataType::Notice(Notice::OfflineFileReceived {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::AddFriendRequest(event) => {
                info!("Q::{} > Event: AddFriendRequest ", &event.user_id);
                BotDataType::Request(Request::AddFriendRequest{
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::AddGroupRequest(event) => {
                info!("G::{} > Q::{} > Event: AddGroupRequest ", &event.group_id,&event.user_id);
                BotDataType::Request(Request::AddGroupRequest{
                    sub_type: GroupAddSubType::new(&event.sub_type),
                    event:event.clone(),
                    bot:bot.clone(),
                })
            },
            EventType::OtherClientOnlineStatusChanges(event) => {
                info!(" Event: OtherClientOnlineStatusChanges ");
                BotDataType::Notice(Notice::OtherClientOnlineStatusChanges {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::EssenceMessage(event) => {
                info!(" Q::{} > Event: EssenceMessage ", &event.operator_id);
                BotDataType::Notice(Notice::EssenceMessage {
                    event,
                    bot:bot.clone(),
                })
            },
            EventType::ApiReturnResult(event) => {
                if event.retcode != 0 {
                    info!("{} > msg:{} > wording:{} ", &event.status, &event.msg, &event.wording);
                }else {
                    info!("{} > Event: ApiReturnResult echo > {}", &event.status, &event.echo);
                }
                BotDataType::Null
            }

        }
    }
}
