use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

/// Generation Channels are used to send and receive data from different threads.
/// Used Dynamic Typing to allow multiple different types to be used.
///
///
/// ### Fields
/// `sender`   -   An optional sender that transmits dialogue responses between threads.
/// `reciever` -   An optional receiver that collects dialogue responses from another thread.
pub struct GenerationChannels<T> {
    pub receiver: Option<Receiver<T>>,
    pub sender: Option<Sender<T>>,
}

/// Uses the Factory Pattern to create default generation channels.
impl<T> Default for GenerationChannels<T> {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel::<T>(2);

        GenerationChannels {
            receiver: Some(receiver),
            sender: Some(sender),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_response_channels() {
        let mut channels = GenerationChannels::default();

        if let Some(sender) = &channels.sender {
            sender.send("Test Message".to_string()).await.unwrap();
        }

        if let Some(receiver) = &mut channels.receiver {
            assert_eq!(receiver.try_recv().unwrap(), "Test Message".to_string());
        }
    }
}
