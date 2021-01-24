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
