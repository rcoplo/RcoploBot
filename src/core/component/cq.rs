use std::fmt::Display;
use std::process::id;
use crate::core::component::message::{image, text};

pub struct Cq {
    vec:Vec<String>,
}
impl Cq {
    pub fn new() -> Cq {
        Cq{
            vec: vec![],
        }
    }
    pub fn set<M : AsRef<str> + Display>(&mut self,cq:Vec<M>) -> &mut Cq {
        for cq in cq{
            self.vec.push( format!("{}",<M as Into<M>>::into(cq)));
        }
        self
    }

    pub fn image<M : AsRef<str>+ Display>(&mut self,file:M) -> &mut Cq {
        self.vec.push(format!("[CQ:image,file={}]", <M as Into<M>>::into(file)));
        self
    }

    pub fn text<M : AsRef<str>+ Display>(&mut self,text: M) -> &mut Cq {
        self.vec.push(format!("{}",<M as Into<M>>::into(text)));
        self
    }

    pub fn face(&mut self,id: i32) -> &mut Cq {
        self.vec.push(format!("[CQ:face,id={}]", id));
        self
    }

    pub fn record<M : AsRef<str>+ Display>(&mut self,file: M) -> &mut Cq {
        self.vec.push(format!("[CQ:record,file={}]", <M as Into<M>>::into(file)));
        self
    }

    pub fn video<M : AsRef<str>+ Display>(&mut self,file: M) -> &mut Cq {
        self.vec.push(format!("[CQ:video,file={}]", <M as Into<M>>::into(file)));
        self
    }

    pub fn video_all<M : AsRef<str>+ Display>(&mut self,file: M, cover: M) -> &mut Cq {
        self.vec.push(format!("[CQ:video,file={},cover={}]", <M as Into<M>>::into(file), <M as Into<M>>::into(cover)));
        self
    }

    pub fn at(&mut self,qq: i64) -> &mut Cq {
        self.vec.push(format!("[CQ:at,qq={}]", qq));
        self
    }

    pub fn at_name<M : AsRef<str>+ Display>(&mut self, qq: i64, name: M) -> &mut Cq {
        self.vec.push(format!("[CQ:at,qq={},name={}]", qq, <M as Into<M>>::into(name)));
        self
    }

    pub fn at_all(&mut self,) -> &mut Cq {
        self.vec.push(format!("[CQ:at,qq=all]"));
        self
    }

    pub fn share<M : AsRef<str>+ Display>(&mut self, url: M, title: M) -> &mut Cq {
        self.vec.push(format!("[CQ:share,url={},title={}]",
                              <M as Into<M>>::into( url),
                              <M as Into<M>>::into(title)));
        self
    }

    pub fn share_all<M : AsRef<str>+ Display>(&mut self, url: M, title: M, content: M, image: M) -> &mut Cq {
        self.vec.push(format!("[CQ:share,url={},title={},content={},image={}]",
                              <M as Into<M>>::into(url),
                              <M as Into<M>>::into( title),
                              <M as Into<M>>::into(content),
                              <M as Into<M>>::into(image)));
        self
    }

    pub fn music<M : AsRef<str>+ Display>(&mut self,r#type: M, id: i64) -> &mut Cq {
        self.vec.push(format!("[CQ:music,type={},id={}]",
                              <M as Into<M>>::into(r#type),
                              id));
        self
    }

    pub fn music_all<M : AsRef<str>+ Display>(&mut self,r#type: M, url: M, audio: M, title: M, content: M, image: M) -> &mut Cq {
        self.vec.push(format!("[CQ:music,type={},url={},audio={},title={},content={},image={}]",
                              <M as Into<M>>::into(r#type),
                              <M as Into<M>>::into(url),
                              <M as Into<M>>::into(audio),
                              <M as Into<M>>::into(title),
                              <M as Into<M>>::into(content),
                              <M as Into<M>>::into(image)));
        self
    }

    pub fn reply<M : AsRef<str>+ Display>(&mut self,id: i64) -> &mut Cq {
        self.vec.push(format!("[CQ:reply,id={}]", id));
        self
    }

    pub fn reply_all<M : AsRef<str>+ Display>(&mut self,qq: i64, text: M) -> &mut Cq {
        self.vec.push(format!("[CQ:reply,text={},qq={}]",  <M as Into<M>>::into(text), qq));
        self
    }

    pub fn poke(&mut self,qq: i64) -> &mut Cq {
        self.vec.push(format!("[CQ:poke,qq={}]", qq));
        self
    }

    pub fn redbag<M : AsRef<str>+ Display>(&mut self,title: M) -> &mut Cq {
        self.vec.push(format!("[CQ:redbag,title={}]",  <M as Into<M>>::into(title)));
        self
    }
    pub fn meow_err(&mut self)  {
        self.text("喵...");
    }

    pub fn meow_ok(&mut self) {
        self.text("喵!");
    }

    pub fn meow_warn(&mut self) {
        self.text("喵?");
    }

    pub fn end(self) -> String {
        let mut string = String::new();
        for str in self.vec {
            string.push_str(str.as_str());
        }
        string
    }
}
