
use log::info;
use tracing::{Event, event};

use crate::core::component::message::{at, text};

use crate::core::notice::Notice;
use crate::core::request::{GroupAddSubType, Request};

use crate::service::CONTEXT;

//同意 申请入群请求
pub async fn group_handle_module(request: &mut Request) {
    request.set_group_add_request(true, "").await;
}

//同意添加好友请求
pub async fn friend_handle_module(request: &mut Request) {
    request.set_friend_add_request(true, "").await;
}

//群成员变动
pub async fn group_change_handle(notice: &mut Notice){
    let mut notice = notice.clone();
    if let Notice::GroupMemberDecrease {..} = notice {
        member_change_decrease_handle_module(&mut notice).await;
    }
    if let Notice::GroupMemberIncrease {..} = notice {
        member_change_increase_handle_module(&mut notice).await;
    }


}

//群成员增加
pub async fn member_change_increase_handle_module(increase: &mut Notice) {
    if let Notice::GroupMemberIncrease { event,..} = increase{
        let user_id = event.user_id;
        if user_id != CONTEXT.bot_config.bot_id {
            increase.send_group_msg(vec![at(&user_id), text(" 欢迎大佬入群~~")]).await;

        }
    }

}

//群成员减少
pub async fn member_change_decrease_handle_module(decrease: &mut Notice) {

    match decrease {
        Notice::GroupMemberDecrease { event,.. } => {
            let event = event.clone();
            let stranger = decrease.get_stranger_info().await.unwrap();
            if event.sub_type.eq(&"leave".to_string()){
                decrease.send_group_msg(vec![text(&stranger.nickname), text(" 离开了我们...")]).await;
            }else if event.sub_type.eq(&"kick".to_string()){
                let stranger_operator = decrease.get_stranger_info_user(event.operator_id).await.unwrap();
                decrease.send_group_msg(vec![text(&stranger.nickname), text(format!(" 被{} 踢了...",stranger_operator.nickname))]).await;
            }
        }
        _ => {}
    }



}