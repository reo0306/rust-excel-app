use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename = "No.")]
    pub no: i32,
    #[serde(rename = "商品名")]
    pub product: String,
    #[serde(rename = "価格")]
    pub price: i32,
    #[serde(rename = "税込み(持ち帰り）")]
    pub tax_price_eight: i32,
    #[serde(rename = "税込み(店内)")]
    pub tax_price_ten: i32,
    pub sum: Option<i32>,
}