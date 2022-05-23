use reqwest::blocking::{self, Client};
use reqwest::blocking::RequestBuilder;
use reqwest::header::AUTHORIZATION;

pub trait ApiBase {
    fn call(&self);
}

pub struct chatPostMsg {
}

// 一応関連関数は分けておく。
impl chatPostMsg {
    pub fn new(token:&str,channel:&str,text:&str) {
        println!("to-do : change");
        match reqwest::blocking::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .header(AUTHORIZATION, String::from("Bearer ")+token)
            .query(&[("channel",channel),("text",text)])
            .send()
        {
            Ok(mut response) => {
                println!("response deteails");
                let mut buf = vec![];
                response.copy_to(&mut buf).unwrap();
                println!("{}", std::str::from_utf8(&buf).unwrap());
                for h in response.headers_mut() {
                    println!("h:{} v:{}",&h.0.as_str(),&h.1.to_str().unwrap());
                }
            },
            Err(_) => println!("error..."),

        }    
    }
}

impl ApiBase for chatPostMsg {
    fn call(&self) {
        println!("chatPostMsg SUCCESS");
    }
}