use serde::{Deserialize, Deserializer};
use time::{Duration, OffsetDateTime};

pub(crate) fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let ts = i64::deserialize(deserializer)?;
    let dt = OffsetDateTime::from_unix_timestamp(ts).map_err(serde::de::Error::custom)?;
    Ok(dt)
}

pub(crate) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i64::deserialize(deserializer)?;
    let d = Duration::new(s, 0);
    Ok(d)
}
