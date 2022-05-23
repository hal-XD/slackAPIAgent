use clap::{Command,Arg};

use crate::slackAPI::ApiBase;

mod slackAPI;
mod constants;

fn main() {
    println!("SlACK API AGENT");
    let matches = Command::new("slack API")
        .version("1.0")
        .author("hal")
        .about("slcak api agent")
        .subcommand(
            Command::new("agent")
                .about("agent mode. call slack api.")
                .arg(
                Arg::new("token") 
                    .short('t')
                    .id("token")
                    .takes_value(true)
                    .env("SLACK_API_TOKEN")
                )
                .subcommand(
                    Command::new("chatPostMessage")
                        .about("chatPostMessage API")
                        .arg(
                            Arg::new("channel")
                                .short('c')
                                .takes_value(true)
                                .required(true)
                                .help("specify slack channel that is posted message")
                        )
                        .arg(
                            Arg::new("text")
                                .short('t')
                                .takes_value(true)
                                .help("text")
                        )
                )
        )
        .subcommand(
            Command::new("register")
                .about("register mode. can register slack token.")
        )
        .get_matches();
    let subcommand = matches.subcommand();
    match subcommand {
        Some(("agent",s_matches)) => {
            let token = s_matches
                .value_of("token")
                .expect("token parameter is necessary. please set -s or SLACK_API_TOKEN");
            match s_matches.subcommand() {
            Some((constants::CHAT_POST_MESSAGE,ss_matches)) => {
                println!("start chatPostMessage calling");
                let channel = ss_matches.value_of("channel").expect("something bad while getting channel argument.");
                let text = ss_matches.value_of("text").expect("something bad while getting text argument.");
                let api = slackAPI::chatPostMsg::new(token,channel,text);
                //api.call();
            },
            Some((not_support_api,ssmatches)) => panic!("Sadly {} have not been supported.",not_support_api),
            _ => panic!("Something strange occurred"),
        }} , 
        Some(("register",s_matches)) => {println!("now creating..")} , 
        _ => unreachable!("clap"),
    }
}
