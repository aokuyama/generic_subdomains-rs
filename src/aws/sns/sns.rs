use crate::aws::{Config, SnsClient};
use aws_sdk_sns::{model::MessageAttributeValue, Client};

impl SnsClient {
    pub fn new<'a>(config: &'a Config, topic_arn: String, default_channel: Option<String>) -> Self {
        let client = Client::new(config.sdk());
        SnsClient {
            client,
            topic_arn,
            default_channel,
        }
    }

    pub async fn publish(&self, subject: &str, message: &str) {
        publish(
            &self.client,
            &self.topic_arn,
            subject,
            message,
            &self.default_channel,
        )
        .await;
    }
}

pub async fn publish(
    client: &Client,
    topic_arn: &str,
    subject: &str,
    message: &str,
    channel: &Option<String>,
) {
    let publish = client
        .publish()
        .topic_arn(topic_arn)
        .subject(subject)
        .message(message);
    let publish = match channel {
        Some(channel) => {
            let builder = MessageAttributeValue::builder();
            let mav = builder
                .set_data_type(Some("String".to_owned()))
                .set_string_value(Some(channel.to_owned()))
                .set_binary_value(None)
                .build();
            publish.message_attributes("channel", mav)
        }
        None => publish,
    };
    match publish.send().await {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
}
