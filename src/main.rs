use std::env;

use transmission_rpc::{
    types::{BasicAuth, Id, TorrentAction, TorrentGetField},
    TransClient,
};

async fn auth() -> TransClient {
    let user = env::var("TUSER").expect("set TUSER environment variable");
    let pass = env::var("TPASS").expect("set TPASS environment variable");

    let basic_auth = BasicAuth {
        user: String::from(user),
        password: String::from(pass),
    };

    let url = String::from(env::var("TURL").expect("set TURL environment variable"));
    let url = url.parse().expect("can't parse URL");

    TransClient::with_auth(url, basic_auth)
}

#[tokio::main]
async fn main() {
    let mut client = auth().await;
    let mut ids: Vec<Id> = Vec::new();

    let res = client
        .torrent_get(
            Some(vec![
                TorrentGetField::Id,
                TorrentGetField::IsPrivate,
                TorrentGetField::LeftUntilDone,
            ]),
            None,
        )
        .await
        .expect("can't connect to Transmission");

    for t in res.arguments.torrents.iter().filter(|t| {
        t.id.as_ref().is_some()
            && !t.is_private.as_ref().unwrap_or(&false)
            && t.left_until_done.as_ref().unwrap_or(&1) == &0
    }) {
        let id = t.id.as_ref().unwrap();
        ids.push(Id::Id(*id));
    }

    if ids.len() == 0 {
        return;
    }

    let res = client.torrent_action(TorrentAction::Stop, ids).await;
    println!("result: {}", res.is_ok());
}
