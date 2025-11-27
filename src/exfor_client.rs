use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Section {
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
pub struct CrossSectionPoint {
    #[serde(alias = "E")]
    pub energy: f64,
    #[serde(alias = "Sig")]
    pub cross_section: f64,
    #[serde(alias = "dSig")]
    pub uncertainty: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrossSectionDataset {
    pub id: String,
    #[serde(alias = "FILE")]
    pub file: String,
    #[serde(alias = "dataType")]
    pub data_type: String,
    #[serde(alias = "LIBRARY")]
    pub library: String,
    #[serde(alias = "TARGET")]
    pub target: String,
    #[serde(alias = "TEMP")]
    pub temp: f64,
    #[serde(alias = "NSUB")]
    pub nsub: u32,
    #[serde(alias = "MAT")]
    pub mat: u32,
    #[serde(alias = "MF")]
    pub mf: u32,
    #[serde(alias = "MT")]
    pub mt: u32,
    #[serde(alias = "REACTION")]
    pub reaction: String,
    #[serde(alias = "COLUMNS")]
    pub columns: Vec<String>,
    #[serde(alias = "defaultInterpolation")]
    pub default_interpolation: String,
    #[serde(alias = "nPts")]
    pub n_pts: u32,
    #[serde(alias = "pts")]
    pub points: Vec<CrossSectionPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrossSectionResponse {
    pub format: String,
    pub now: String,
    pub program: String,
    pub datasets: Vec<CrossSectionDataset>,
}

#[derive(Debug, Deserialize, Serialize)]
struct E4Response {
    pub format: String,
    pub now: String,
    pub program: String,
    pub req: u32,
    pub sections: Vec<Section>,
}

async fn fetch_data(
    target: &str,
    reaction: &str,
    quantity: &str,
) -> Result<E4Response, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www-nds.iaea.org/exfor/e4list?Target={}&Reaction={}&Quantity={}&json",
        target, reaction, quantity
    );

    let response = reqwest::get(&url).await?.json::<E4Response>().await?;
    Ok(response)
}

fn filter_by_library(response: E4Response, lib_name: &str) -> E4Response {
    let filtered_sections: Vec<Section> = response
        .sections
        .into_iter()
        .filter(|section| section.lib_name == lib_name)
        .collect();

    E4Response {
        format: response.format,
        now: response.now,
        program: response.program,
        req: response.req,
        sections: filtered_sections,
    }
}

pub async fn fetch_cross_section(
    target: &str,
    reaction: &str,
    lib_name: &str,
) -> Result<CrossSectionResponse, Box<dyn std::error::Error>> {
    let quantity = "SIG";
    let response = fetch_data(target, reaction, quantity).await?;
    let filtered = filter_by_library(response, lib_name);

    if filtered.sections.is_empty() {
        return Err("No sections found for the specified library".into());
    }

    let section = &filtered.sections[0];
    let url = format!(
        "https://www-nds.iaea.org/exfor/e4sig?SectID={}&PenSectID={}&json",
        section.sect_id, section.pen_sect_id
    );

    let cross_section_data = reqwest::get(&url)
        .await?
        .json::<CrossSectionResponse>()
        .await?;
    Ok(cross_section_data)
}
