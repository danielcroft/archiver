use serde::ser::{Serialize, Serializer};
use reqwest;
use failure::Error;

static MESSAGE_API_URL: &'static str = "https://api.pushover.net/1/messages.json";

#[derive(Debug)]
pub enum Priority {
    NoNotification,
    QuietNotification,
    HighPriority,
    RequireConfirmation,
}

impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pri = match self {
            Priority::NoNotification => -2,
            Priority::QuietNotification => -1,
            Priority::HighPriority => 1,
            Priority::RequireConfirmation => 2,
        };
        serializer.serialize_i8(pri)
    }
}

#[derive(Serialize,Debug)]
pub struct PushoverRequest<'a> {
    token: &'a str,
    user: String,
    message: String,
    // attachment - an image attachment to send with the message; see attachments for more information on how to upload files
    // device - your user's device name to send the message directly to that device, rather than all of the user's devices (multiple devices may be separated by a comma)
    title: Option<String>,
    url: Option<String>,
    url_title: Option<String>,
    priority: Option<Priority>,
    // sound - the name of one of the sounds supported by device clients to override the user's default sound choice
    // timestamp - a Unix timestamp of your message's date and time to display to the user, rather than the time your message is received by our API
}

impl<'a> PushoverRequest<'a> {
    pub fn send(self) -> Result<reqwest::Response, Error> {
        let client = reqwest::Client::new();
        client.post(MESSAGE_API_URL)
            .form(&self)
            .send()
            .map_err(|e| format_err!("HTTP error: {:?}", e))
    }
}

pub struct Pushover {
    token: String,
}

impl Pushover {
    pub fn new(token: String) -> Pushover {
        Pushover {
            token,
        }
    }

    pub fn request(&self, user: String, message: String) -> PushoverRequest {
        PushoverRequest {
            token: &self.token,
            user,
            message,
            title: None,
            url: None,
            url_title: None,
            priority: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    #[ignore]
    fn test_sends_notification() {
        fn inner() -> Result<(), Error> {
            let pushover = Pushover::new(env::var("ARCHIVER_TEST_PUSHOVER_KEY").expect("Didn't provide test key"));
            let user_key: String = "redacted".into();
            pushover.request(user_key, "hi there".into()).send()?;
            Ok(())
        }

        if let Err(e) = inner() {
            panic!("{:?}", e);
        }
    }
}