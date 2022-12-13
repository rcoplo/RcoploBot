use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use futures_util::lock::Mutex;
use serde_json::{json, to_string, Value};
use crate::core::bot::{Bot, ApiResult, BotDataType};
use crate::core::component::event::{EventType, PostType};
use crate::core::component::message::text;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

use crate::core::group::Group;
use crate::core::network::ws::ws_handler::{ ws_handle};
use crate::service::CONTEXT;

#[derive(Debug,Clone)]
pub struct WsClient ;

impl WsClient {
    pub async fn run()  {
        let config = CONTEXT.bot_config.clone();

        let url = match config.access_token {
            Some(token) => {
                let url = config.url.expect("[Bot] websocket url is null");
                url::Url::parse_with_params(url.as_str(), &[("access_token", token)]).unwrap()
            }
            None => {
                let url = config.url.expect("[Bot] websocket url is null");
                url::Url::parse(url.as_str()).unwrap()
            }
        };
        let (api_sender, mut api_receiver) = mpsc::channel(1024);

        let (ws_steam, _) = connect_async(url).await.unwrap();

        let (mut ws_out, mut ws_in) = ws_steam.split();

        let resp_promises = Arc::new(Mutex::new(HashMap::new()));
        let next = ws_in.next().await.unwrap().unwrap().clone();
        // 获取一下bot id
        let value = match next {
            Message::Text(msg) => {
               Some(serde_json::from_str::<Value>(msg.as_str()).unwrap())
            }
           _ => None
        };
        let mut bot = Bot {
            bot_id: value.unwrap()["self_id"].as_i64().unwrap(),
            api_sender: mpsc::Sender::clone(&api_sender),
            resp_promises: resp_promises.clone(),
        };

        info!("[Bot] [client] WebSocket handshake has been successfully completed");


        // 发送 api
        let mut send_task = tokio::spawn(async move {
            while let Some(frame) = api_receiver.recv().await {
                let data = frame.data.unwrap();
                if ws_out.send(Message::text(to_json(data))).await.is_err() {
                    break;
                }
            }
        });

        // 获取 api
        let mut recv_task = tokio::spawn(async move {
            while let Some(msg) = ws_in.next().await {
                let msg = msg.unwrap();
                match msg {
                    Message::Text(msg) => {
                        let json: Value = serde_json::from_str(msg.as_str()).unwrap();
                        let mut bot = bot.clone();
                        //事件处理
                        ws_handle(json,&mut bot).await;
                    }
                    Message::Close(_) => { break; }
                    _ => {
                        break;
                    }
                }
            }
        });
        tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
        };
    }

}

fn to_json(v: Value) -> String {
    to_string(&v).unwrap()
}