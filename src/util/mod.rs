pub mod file;
mod image_utils;
pub mod http_utils;


use tracing::info;
pub use image_utils::*;
///
/// # 有五种匹配参数
/// * {at}   匹配ai 群友
/// * {at_bot}   匹配ai bot
/// * {bot_name}   匹配bot名字
/// * {msg}   匹配 字符串
/// * {reply}   匹配 回复 群友
/// * {reply_bot}    匹配 回复 bot
///
/// 建议组合使用
pub fn parameter_to_regular(
    // 指令集
    vec:&Vec<&str>,
    // bot_name
    bot_name:&Vec<String>,
    // bot_id
    bot_id:&i64,
    // 说的话, 前面不会有 at ,回复之类的格式化信息
    msg:&String,
) ->Vec<String>{
    let mut v = vec![];
    let mut string = String::new();
    let mut string_msg = String::new();
    string.push_str("(");
    for name in bot_name {
        string.push_str(format!("({})|",name).as_str());
        // 将可能有bot名字的字符串清除
        let str = msg.replace(name, "");
        string_msg = str;
    }
    string.push_str("()");
    // 自定义bot 名字的集合 正则是这样的: ((bot_name)|(bot_name)|(bot_name))
    let string_bot_name = string.replace("|()", ")");
    for x in vec {
        let string = x
            .replace("{at_bot}", format!(r"at\[{},null] ", &bot_id).as_str())
            .replace("{at}", format!(r"at\[(.*),(.*)] ").as_str())
            .replace("{bot_name}", string_bot_name.as_str())
            .replace("{msg}", &string_msg)
            .replace("{reply}", format!(r"reply\[(.*),(.*),(.*)] ").as_str())
            .replace("{reply_bot}", format!(r"reply\[(.*),{},(.*)] ", &bot_id).as_str());
        v.push(string);
    }
    v
}

