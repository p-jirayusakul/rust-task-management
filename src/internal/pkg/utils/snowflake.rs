use sonyflake::Sonyflake;
use mockall::automock;

#[automock]
pub trait Snowflake {
    fn generate(&self) -> u64;
}

pub struct SnowflakeImpl {
    snowflake_id: Sonyflake,
}

impl SnowflakeImpl {
    pub fn new(snowflake_id: Sonyflake) -> Self {
        Self { snowflake_id }
    }
}

impl Snowflake for SnowflakeImpl {
    fn generate(&self) -> u64 {
        self.snowflake_id.next_id().expect("Failed to generate next Sonyflake ID")
    }
}