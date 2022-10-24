use aws_config::{meta::region::RegionProviderChain, SdkConfig};

use super::Config;

impl Config {
    pub async fn new(fallback: &'static str) -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else(fallback);
        let sdk_config = aws_config::from_env().region(region_provider).load().await;
        Config { sdk_config }
    }
    pub fn sdk(&self) -> &SdkConfig {
        &self.sdk_config
    }
}
