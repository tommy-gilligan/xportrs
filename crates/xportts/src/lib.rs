use xportrs::{Xpt, Agency, Severity, Issue};

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Dataset(xportrs::Dataset);

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Column(xportrs::Column);

#[derive(Tsify, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ColumnData(xportrs::ColumnData);

#[wasm_bindgen]
impl ColumnData {
    pub fn from_numeric(values: Vec<JsValue>) -> Self {
        Self(xportrs::ColumnData::F64(
            values
                .iter()
                .map(|value| {
                    if value.is_null_or_undefined() {
                        None
                    } else {
                        value.as_f64()
                    }
                })
                .collect(),
        ))
    }

    pub fn from_string(values: Vec<JsValue>) -> Self {
        Self(xportrs::ColumnData::String(
            values
                .iter()
                .map(|value| {
                    if value.is_null_or_undefined() {
                        None
                    } else {
                        value.as_string()
                    }
                })
                .collect(),
        ))
    }
}

#[wasm_bindgen]
impl Column {
    pub fn new(name: &str, data: ColumnData) -> Self {
        Self(xportrs::Column::new(name, data.0))
    }
}

#[wasm_bindgen]
pub fn column_with_label(column: Column, label: &str) -> Column {
    Column(column.0.with_label(label))
}

#[wasm_bindgen]
pub fn dataset_with_label(dataset: Dataset, label: &str) -> Dataset {
    let mut ds = dataset.0;
    ds.set_label(label);
    Dataset(ds)
}

#[wasm_bindgen]
impl Dataset {
    pub fn new(domain_code: &str, columns: Vec<Column>) -> Self {
        let cols: Vec<xportrs::Column> = columns.into_iter().map(|c| c.0).collect();
        Self(xportrs::Dataset::new(domain_code, cols).unwrap())
    }
}

#[wasm_bindgen]
pub fn to_xpt(dataset: Dataset) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    Xpt::writer(dataset.0.clone())
        .finalize()
        .unwrap()
        .write_to(&mut buf)
        .unwrap();
    buf
}
