//------------------------------------------------------------------------------
// vfx-rs 2021
//------------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

use std::string::String;
use std::sync::Arc;
use std::vec::Vec;

//------------------------------------------------------------------------------
// Item
//------------------------------------------------------------------------------
#[derive(Debug, serde::Deserialize)]
pub enum Item {
    Function {},
    Record {},
}

#[derive(Debug, serde::Deserialize)]
pub struct AST {
    #[serde(rename = "$value")]
    pub items: Vec<std::sync::Arc<Item>>,
}
