use super::Error;
use crate::aws::{Config, SnsFifoClient};
use aws_sdk_sns::{model::MessageAttributeValue, Client};
use sha2::{Digest, Sha512};

impl SnsFifoClient {
    pub fn new<'a>(config: &'a Config, topic_arn: String) -> Self {
        let client = Client::new(config.sdk());
        SnsFifoClient { client, topic_arn }
    }

    pub async fn publish(
        &self,
        group_id: &str,
        message: &str,
        subject: Option<&str>,
    ) -> Result<(), Error> {
        let subject = match subject {
            Some(s) => s,
            None => group_id,
        };
        publish(&self.client, &self.topic_arn, subject, message, group_id).await
    }
}

pub async fn publish(
    client: &Client,
    topic_arn: &str,
    subject: &str,
    message: &str,
    group_id: &str,
) -> Result<(), Error> {
    let deduplication_id = create_deduplication_id(subject, message, group_id);

    let builder = MessageAttributeValue::builder();
    let mav_group_id = builder
        .set_data_type(Some("String".to_owned()))
        .set_string_value(Some(group_id.to_owned()))
        .set_binary_value(None)
        .build();

    let publish = client
        .publish()
        .topic_arn(topic_arn)
        .subject(subject)
        .message(message)
        .message_group_id(group_id)
        .message_deduplication_id(deduplication_id)
        .message_attributes("group_id", mav_group_id);

    match publish.send().await {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::SdkError(err.to_string())),
    }
}

fn create_deduplication_id(subject: &str, message: &str, group_id: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(format!("{:?}/{:?}/{:?}", subject, message, group_id));
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use crate::aws::sns::fifo::create_deduplication_id;
    #[test]
    fn it_id_generated_by_all_content() {
        let a = create_deduplication_id("a", "b", "c");
        assert_eq!(a, create_deduplication_id("a", "b", "c"));
        assert_ne!(a, create_deduplication_id("d", "b", "c"));
        assert_ne!(a, create_deduplication_id("a", "d", "c"));
        assert_ne!(a, create_deduplication_id("a", "b", "d"));
    }
    #[test]
    fn it_id_generated_by_sha52() {
        let a = create_deduplication_id("a", "b", "c");
        assert_eq!(128, a.chars().count());
        let b = create_deduplication_id("c", "b", "c");
        assert_eq!(128, b.chars().count());
    }
}
