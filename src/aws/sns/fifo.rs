use crate::aws::{Config, SnsFifoClient};
use aws_sdk_sns::Client;

impl SnsFifoClient {
    pub fn new<'a>(config: &'a Config, topic_arn: String) -> Self {
        let client = Client::new(config.sdk());
        SnsFifoClient { client, topic_arn }
    }

    pub async fn publish(&self, group_id: &str, message: &str, subject: Option<&str>) {
        let subject = match subject {
            Some(s) => s,
            None => group_id,
        };
        publish(&self.client, &self.topic_arn, subject, message, group_id).await;
    }
}

pub async fn publish(
    client: &Client,
    topic_arn: &str,
    subject: &str,
    message: &str,
    group_id: &str,
) {
    let publish = client
        .publish()
        .topic_arn(topic_arn)
        .subject(subject)
        .message(message)
        .message_group_id(group_id);
    match publish.send().await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
}
