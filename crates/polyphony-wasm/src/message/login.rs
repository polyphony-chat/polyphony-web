use crate::client::Client;
use crate::screen::Screen;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LoginMessage {
    Test(String),
}

impl LoginMessage {
    pub fn update(client: &mut Client, message: Self) -> bool {
        let Screen::Login(screen) = &mut client.screen else {
            return false;
        };
        match message {
            LoginMessage::Test(str) => {
                screen.text = str;
                true
            }
        }
    }
}
