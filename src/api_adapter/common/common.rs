use http::StatusCode;
use serde::{Serialize, Serializer};

pub struct CustomHttpStatusCode(pub StatusCode);
impl Serialize for CustomHttpStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.0.as_u16())
    }
}

