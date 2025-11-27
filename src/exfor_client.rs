use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    #[serde(alias = "Targ")]
    pub target: String,
    #[serde(alias = "ZT")]
    pub z: u32,
    #[serde(alias = "AT")]
    pub a: u32,
    #[serde(alias = "NSUB")]
    pub nsub: u32,
    #[serde(alias = "MT")]
    pub mt: u32,
    #[serde(alias = "MF")]
    pub mf: u32,
    #[serde(alias = "R")]
    pub r: String,
    #[serde(alias = "RC")]
    pub rc: String,
    #[serde(alias = "EvalID")]
    pub eval_id: u32,
    #[serde(alias = "SectID")]
    pub sect_id: u32,
    #[serde(alias = "PenSectID")]
    pub pen_sect_id: u32,
    #[serde(alias = "LibID")]
    pub lib_id: u32,
    #[serde(alias = "LibName")]
    pub lib_name: String,
    #[serde(alias = "DATE")]
    pub date: String,
    #[serde(alias = "AUTH")]
    pub auth: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct E4Response {
    pub format: String,
    pub now: String,
    pub program: String,
    pub req: u32,
    pub sections: Vec<Section>,
}

pub async fn fetch_data(target: &str, reaction: &str, quantity: &str) -> E4Response {
    let body = reqwest::get(format!(
        "https://www-nds.iaea.org/exfor/e4list?Target={}&Reaction={}&Quantity={}&json",
        target, reaction, quantity
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let v: E4Response = serde_json::from_str(body.as_str()).unwrap();
    return v;
}
