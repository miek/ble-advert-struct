extern crate base64;
#[macro_use]
extern crate base64_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

base64_serde_type!(serde_base64, base64::URL_SAFE_NO_PAD);

#[derive(Debug, Serialize, Deserialize)]
pub struct BLEAdvert {
    pub listener: String,
    #[serde(with = "serde_base64")]
    pub manufacturer_data: Vec<u8>,
    pub mac: String,
    pub time: std::time::SystemTime,
}
