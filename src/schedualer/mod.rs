use std::time::Instant;

pub struct ValueProperties {
    pub value: String,
    pub insert_time: Instant,
    pub expire_time: Option<u128>,
}

impl ValueProperties {
    pub fn new(value: String, insert_time: Instant, expire_time: Option<u128>) -> Self {
        Self {
            value,
            insert_time,
            expire_time,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            if Instant::now().duration_since(self.insert_time).as_millis() > expire_time {
                return true;
            }
        }
        false
    }
}

