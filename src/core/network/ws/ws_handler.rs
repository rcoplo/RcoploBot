use std::collections::HashMap;
use std::sync::Arc;
use futures_util::lock::Mutex;
use tokio::sync::{mpsc};
use tokio::sync::mpsc::{
   Sender
};
use serde_json::Value;
use crate::bot::event_handler;
use crate::core::bot::{ApiResult, Bot, BotDataType};
use crate::core::component::event::EventType;
use crate::core::friend::Friend;
use crate::core::group::Group;
use crate::core::network::DefaultHandler;
use crate::core::notice::Notice;
use crate::core::request::Request;


pub async fn  ws_handle(json: Value, bot:&mut Bot) {
   let mut bot = bot.clone();
    tokio::spawn(async move {
      //将数据 转换成具体结构体
      let event = EventType::get_event(json);
      match event {
         Some(event) => {
            match event {
               EventType::ApiReturnResult(event) => {
                  // 返回消息处理
                  if let Some(api_resp_sender) = bot.resp_promises.lock().await.remove(event.echo.as_str()) {
                     let data = event.data.clone();
                     if event.status.contains("ok") {
                        if data["message_id"].is_null() {
                           api_resp_sender.send(ApiResult {
                              echo: event.echo,
                              ok: true,
                              data: Some(data),
                              return_data: Ok(()),
                              message_id: 0,
                           }).expect("返回Api错误");
                        } else {
                           api_resp_sender.send(ApiResult {
                              echo: event.echo,
                              ok: true,
                              data: Some(data.clone()),
                              return_data: Ok(()),
                              message_id: data["message_id"].as_i64().unwrap(),
                           }).expect("返回Api错误");
                        }
                     } else {
                        api_resp_sender.send(ApiResult {
                           echo: event.echo.clone(),
                           ok: false,
                           data: Some(data),
                           return_data: Err(event.clone()),
                           message_id: 0,
                        }).expect("返回Api错误");
                     }
                  }
               }
               _ => {

                  let mut data_type = DefaultHandler::handle(event, &mut bot).await;

                  event_handler(&mut data_type).await;
               }
            }
         }
         None => {
            // 什么事件都没匹配到...
         }
      }
   });
}