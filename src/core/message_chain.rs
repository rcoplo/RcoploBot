use std::collections::HashMap;
use std::sync::Arc;
use regex::RegexSet;
use tracing::{event, info};
use crate::core::bot::{Bot, message_handle};
use crate::core::component::message::Message;
use crate::service::CONTEXT;
use crate::util::parameter_to_regular;

#[derive(Debug, Clone)]
pub struct MessageChain {
    ///格式化过的 message
    pub message: String,
    /// 以空格分开的/格式化过的 message_list
    pub message_list: Vec<String>,
    ///消息id
    pub message_id: i64,
    /// 消息链
    message_chain: Vec<Message>,
    bot:Bot,
}

impl MessageChain {

    pub fn init(message_id: &i64,message: &Vec<Message>, raw_message: &String,bot:&mut Bot) -> MessageChain {
        let (msg,message_list) = message_handle(message.clone(), raw_message.clone());
        MessageChain{
            message:msg,
            message_list,
            message_id: message_id.clone(),
            message_chain: message.clone(),
            bot: bot.clone(),
        }
    }

    pub fn text< T: AsRef<str>>(&mut self, text: T) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "text".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("text".to_string(),text.as_ref().to_string()),
            ])),
        });
        self
    }

    pub fn face(&mut self, id: i32) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "face".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("id".to_string(), id.to_string()),
            ])),
        });
        self
    }

    pub fn record(&mut self, file: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "record".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), file.to_string()),
            ])),
        });
        self
    }

    pub fn video(&mut self,file: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "video".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), file.to_string()),
            ])),
        });
        self
    }

    pub fn at(&mut self,qq: &i64) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "at".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), qq.to_string()),
            ])),
        });
        self
    }

    pub fn at_all(&mut self,) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "at".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), "all".to_string()),
            ])),
        });
        self
    }

    pub fn at_name(&mut self,qq: &i64, name: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "at".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), qq.to_string()),
                ("name".to_string(), name.to_string()),
            ])),
        });
        self
    }

    pub fn share(&mut self,url: &str, title: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "share".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("url".to_string(), url.to_string()),
                ("title".to_string(), title.to_string()),
            ])),
        });
        self
    }

    pub fn share_all(&mut self,url: &str, title: &str, content: &str, image: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "share".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), url.to_string()),
                ("title".to_string(), title.to_string()),
                ("content".to_string(), content.to_string()),
                ("image".to_string(), image.to_string()),
            ])),
        });
        self
    }

    pub fn music(&mut self,r#type: &str, id: &i64) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "music".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("type".to_string(), r#type.to_string()),
                ("id".to_string(), id.to_string()),
            ])),
        });
        self
    }

    pub fn music_all(&mut self,r#type: &str, url: &str, audio: &str, title: &str, content: &str, image: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "music".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("type".to_string(), r#type.to_string()),
                ("url".to_string(), url.to_string()),
                ("audio".to_string(), audio.to_string()),
                ("title".to_string(), title.to_string()),
                ("content".to_string(), content.to_string()),
                ("image".to_string(), image.to_string()),
            ])),
        });
        self
    }

    pub fn image<T:  AsRef<str>>(&mut self,url: T) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "image".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), url.as_ref().to_string()),
            ])),
        });
        self
    }

    pub fn reply(&mut self,message_id: &i64) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "reply".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("id".to_string(), message_id.to_string()),
            ])),
        });
        self
    }

    pub fn reply_text(&mut self,qq: &i64, text: &str) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "reply".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), qq.to_string()),
                ("text".to_string(), text.to_string()),
            ])),
        });
        self
    }

    pub fn redbag(&mut self,title: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "redbag".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("title".to_string(), title.to_string()),
            ])),
        });
        self
    }

    pub fn poke(&mut self,qq: &i64) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "poke".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), qq.to_string()),
            ])),
        });
        self
    }

    pub fn gift(&mut self,qq: &i64, id: i32) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "gift".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("qq".to_string(), qq.to_string()),
                ("id".to_string(), id.to_string()),
            ])),
        });
        self
    }

    pub fn forward_ref(&mut self,id: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "forward".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("id".to_string(), id.to_string()),
            ])),
        });
        self
    }

    pub fn forward_node(&mut self,name: &str, uin: &i64, content: Vec<Message>) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "node".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("name".to_string(), name.to_string()),
                ("uin".to_string(), uin.to_string()),
                ("content".to_string(), serde_json::to_string(&content).unwrap()),
            ])),
        });
        self
    }

    pub fn node_ref(&mut self,message_id: i32) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "node".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("id".to_string(), message_id.to_string()),
            ])),
        });
        self
    }


    pub fn xml(&mut self,data: &str) -> &mut MessageChain {
        self.message_chain.push(Message {
            r#type: "xml".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("data".to_string(), data.to_string()),
            ])),
        });
        self
    }

    pub fn xml_all(&mut self,data: &str, resid: i32) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "xml".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("data".to_string(), data.to_string()),
                ("resid".to_string(), resid.to_string()),
            ])),
        });
        self
    }


    pub fn json(&mut self,data: &str) -> &mut MessageChain {
        let data = data
            .replace(",", "&#44;")
            .replace("&", "&amp;")
            .replace("[", "&#91;")
            .replace("]", "&#93;");
        self.message_chain.push(Message {
            r#type: "json".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("data".to_string(), data.to_string()),
            ])),
        });
        self
    }

    pub fn json_all(&mut self,data: &str, resid: i32) -> &mut MessageChain {
        let data = data
            .replace(",", "&#44;")
            .replace("&", "&amp;")
            .replace("[", "&#91;")
            .replace("]", "&#93;");
        self.message_chain.push(Message {
            r#type: "json".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("data".to_string(), data.to_string()),
                ("resid".to_string(), resid.to_string()),
            ])),
        });
        self
    }

    pub fn cardimage(&mut self,file: &str) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "cardimage".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), file.to_string()),
            ])),
        });
        self
    }

    pub fn cardimage_all(&mut self,file: &str, minwidth: i32, minheight: i32, maxwidth: i32, maxheight: i32, source: &str, icon: &str) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "cardimage".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("file".to_string(), file.to_string()),
                ("minwidth".to_string(), minwidth.to_string()),
                ("minheight".to_string(), minheight.to_string()),
                ("maxwidth".to_string(), maxwidth.to_string()),
                ("maxheight".to_string(), maxheight.to_string()),
                ("source".to_string(), source.to_string()),
                ("icon".to_string(), icon.to_string()),
            ])),
        });
        self
    }

    pub fn tts(&mut self,text: &str) -> &mut MessageChain {
        self.message_chain.push( Message {
            r#type: "tts".to_string(),
            data: HashMap::<_, _>::from_iter(IntoIterator::into_iter([
                ("text".to_string(), text.to_string()),
            ])),
        });
        self
    }
    pub fn match_command(&self,vec:&Vec<&str>,no_msg:&Vec<&str>) -> bool {
        let bot_name = CONTEXT.bot_config.bot_name.as_ref().unwrap();
        let bot_id = &self.bot.bot_id;
        let mut msg = &self.message;
        let v = parameter_to_regular(
            vec, bot_name, bot_id,&self.msg()
        );
        let v2 = parameter_to_regular(
            no_msg, bot_name, bot_id,&self.msg()
        );

        let result = RegexSet::new(&v);
        let result2 = RegexSet::new(&v2);
        match result {
            Ok(regex) => {
                regex.is_match(msg.as_str()) && !result2.unwrap().is_match(msg.as_str())
            }
            Err(_) => false
        }
    }
    pub fn msg(&self) -> String {
        self.message_list[self.message_list.len() - 1].clone()
    }
}
