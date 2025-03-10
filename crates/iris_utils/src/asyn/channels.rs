use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Channels<T> {
    pub reciever: Option<Receiver<T>>,
    pub sender: Option<Sender<T>>,
}

impl<T> Default for Channels<T> {
    fn default() -> Self {
        let (sender, reciever) = mpsc::channel(2);

        Channels {
            reciever: Some(reciever),
            sender: Some(sender),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channels() {
        let mut channel = Channels::default();

        if let Some(sender) = &channel.sender {
            sender.send("Test message".to_string()).await.unwrap();
        }

        if let Some(reciever) = &mut channel.reciever {
            assert_eq!(reciever.try_recv().unwrap(), "Test message".to_string());
        }
    }
}
