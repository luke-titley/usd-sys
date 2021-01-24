//#[xml_schema(source = "src/castxml.xsd", store_generated_code=Some("castxml.rs"))]

/*
use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(source = "src/castxml.xsd", store_generated_code = "output.rs")]
struct CastXml;
*/

mod schema;

use schema::{CastXML, Item};

use serde_xml_rs::from_reader;

fn main() {
    //let mut file = std::fs::File::open("/Volumes/src/usd-sys/build/bind_stripped.xml")
    let mut file = std::fs::File::open("/Volumes/src/usd-sys/build/bind.xml")
                         .expect("Unable to read file");

    let root : schema::CastXML = from_reader(file).expect("Unable to read xml file");

    println!("{}", root.format);
    println!("{}", root.items.len());

    for item in root.items.iter() {
        println!("Hello");
        match item {
            Item::Field{ id, ..} => {
                println!("Field id: {}", id);
            }
            _ => (),
        }
    }
}
