use std::{env, process::exit};

use ruma_client::{
    self,
    events::{
        collections::all::RoomEvent,
        room::message::{MessageEvent, MessageEventContent, TextMessageEventContent},
    },
    AsyncClient, SyncSettings,
};

async fn login(
    homeserver_url: String,
    username: String,
    password: String,
) -> Result<(), ruma_client::Error> {
    let mut client = AsyncClient::new(&homeserver_url, None).unwrap();

    client.login(username, password, None).await?;
    let response = client.sync(SyncSettings::new()).await?;

    for (_, room) in response.rooms.join {
        for event in room.timeline.events {
            if let RoomEvent::RoomMessage(MessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent { body: msg_body, .. }),
                sender,
                ..
            }) = event
            {
                println!("{}: {}", sender, msg_body);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ruma_client::Error> {
    let (homeserver_url, username, password) =
        match (env::args().nth(1), env::args().nth(2), env::args().nth(3)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => {
                eprintln!(
                    "Usage: {} <homeserver_url> <username> <password>",
                    env::args().next().unwrap()
                );
                exit(1)
            }
        };

    login(homeserver_url, username, password).await
}
