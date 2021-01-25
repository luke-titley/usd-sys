use castxml_schema::{CastXML, Item};
use serde_xml_rs::from_reader;

fn main() {
    //let mut file = std::fs::File::open("/Volumes/src/usd-sys/build/bind_stripped.xml")
    let mut file = std::fs::File::open("/Volumes/src/usd-sys/build/bind.xml")
                         .expect("Unable to read file");

    let root : CastXML = from_reader(file).expect("Unable to read xml file");

    /*
    println!("{}", root.format);
    println!("{}", root.items.len());
    */

    for item in root.items.iter() {
        match item {
            Item::Field{ id, ..} => {
                //println!("Field id: {}", id);
            }
            _ => (),
        }
    }
}
