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

pub(crate) fn deserialize_option_float_to_int<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let f = Option::<f64>::deserialize(deserializer)?;
    Ok(f.map(|x| x as i64))
}

pub(crate) fn deserialize_float_to_int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let f = f64::deserialize(deserializer)?;
    Ok(f as i64)
}
