use clap::ValueEnum;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, TryFromPrimitive)]
#[repr(u16)]
pub enum RCode {
    NoError = 0,
    FormErr = 1,
    ServFail = 2,
    NXDomain = 3,
    NotImp = 4,
    Refused = 5,
    YXDomain = 6,
    YXRRSet = 7,
    NXRRSet = 8,
    NotAuth = 9,
    NotZone = 10,
    DSOTypeName = 11,
    BadVers = 16,
    BadKey = 17,
    BadTime = 18,
    BadMode = 19,
    BadName = 20,
    BadAlg = 21,
    BadTrunc = 22,
    BadCookie = 23,
}

fn deserialize_rcode<'de, D>(deserializer: D) -> Result<RCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u16::deserialize(deserializer)?;
    RCode::try_from(value)
        .map_err(|_| serde::de::Error::custom(format!("invalid RCode type: {}", value)))
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, TryFromPrimitive, ValueEnum)]
#[repr(u16)]
pub enum RRType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    HINFO = 13,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    NSEC = 47,
    HTTPS = 65,
    SPF = 99,
    ANY = 255,
    CAA = 257,
}

fn deserialize_rrtype<'de, D>(deserializer: D) -> Result<RRType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u16::deserialize(deserializer)?;
    RRType::try_from(value)
        .map_err(|_| serde::de::Error::custom(format!("invalid RR type: {}", value)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub name: String,
    #[serde(deserialize_with = "deserialize_rrtype")]
    pub r#type: RRType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub name: String,
    #[serde(deserialize_with = "deserialize_rrtype")]
    pub r#type: RRType,
    #[serde(alias = "TTL")]
    pub ttl: u32,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    #[serde(alias = "Status")]
    #[serde(deserialize_with = "deserialize_rcode")]
    pub status: RCode,
    #[serde(alias = "TC")]
    pub tc: bool,
    #[serde(alias = "RD")]
    pub rd: bool,
    #[serde(alias = "RA")]
    pub ra: bool,
    #[serde(alias = "AD")]
    pub ad: bool,
    #[serde(alias = "CD")]
    pub cd: bool,
    #[serde(alias = "Question")]
    pub question: Vec<Question>,
    #[serde(alias = "Answer")]
    pub answer: Option<Vec<Answer>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub duration: Duration,
}

impl fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "STATUS      : {:?}", self.status)?;
        writeln!(
            f,
            "FLAGS       : rd={} ra={} ad={} cd={} tc={}",
            self.rd, self.ra, self.ad, self.cd, self.tc
        )?;
        writeln!(f, "TIME        : {} ms", self.duration.as_millis())?;
        writeln!(f)?;

        writeln!(f, "[QUESTION]")?;
        for q in &self.question {
            writeln!(f, "{}\tIN\t{:?}", q.name, q.r#type)?;
        }
        writeln!(f)?;

        if let Some(ans) = &self.answer {
            writeln!(f, "[ANSWER]")?;
            for a in ans {
                writeln!(f, "{}\t{}\tIN\t{:?}\t{}", a.name, a.ttl, a.r#type, a.data)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
