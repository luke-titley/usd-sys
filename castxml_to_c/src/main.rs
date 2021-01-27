//------------------------------------------------------------------------------
// Copywrite 2020
//------------------------------------------------------------------------------
mod ast;

use castxml_schema as cs;
use serde_xml_rs::from_reader;

type Items = std::vec::Vec<ast::Item>;

struct Transform<'a, 'b> {
    input: &'a cs::CastXML,
    output: &'b mut ast::AST,
}

impl<'a, 'b> Transform<'a, 'b> {
    // Convert an id reference to an id.
    fn idref_to_item(&self, idref: &str) -> Option<&cs::Item> {
        if &idref[0..1] == "_" {
            let id: u32 = idref[1..].parse().expect("Unable to convert id into int");

            Some(&self.input.items[id as usize])
        } else {
            None
        }
    }

    // Given a string of members:
    // Iterate over each member.
    fn iter_members<F>(&mut self, members: &str, mut f: F)
    where
        F: FnMut(&cs::Item),
    {
        for idref in members.split_ascii_whitespace() {
            if let Some(member) = self.idref_to_item(&idref) {
                f(member)
            }
        }
    }

    fn apply_class(&mut self, kls: &cs::Record) {
        // Loop over all the members of the class.
        // We want to convert the methods/operators to functions and the
        // fields to normal fields.
        if let Some(members) = &kls.members {
            self.iter_members(&members, |item| {
                match item {
                    cs::Item::Method(method) => {
                        println!("method id {}", method.id);
                    }
                    _ => (),
                }
            });
        }
    }

    fn apply_root(&mut self) {
        // Loop over the two main entry points:
        //  - classes
        //  - functions
        //
        for item in self.input.items.iter() {
            match item {
                cs::Item::Class(c) => {
                    self.apply_class(c);
                }
                _ => (),
            }
        }
    }

    fn apply(cast_xml: &cs::CastXML) -> ast::AST {
        let mut result = ast::AST {
            items: std::vec::Vec::new(),
        };

        {
            let mut transform = Transform {
                input: cast_xml,
                output: &mut result,
            };

            transform.apply_root();
        }

        // Return the final ast
        result
    }
}

//------------------------------------------------------------------------------
fn write_headers(root: &ast::AST) {}

//------------------------------------------------------------------------------
fn write_source(root: &ast::AST) {}

//------------------------------------------------------------------------------
fn main() {
    //let mut file = std::fs::File::open("/Volumes/src/usd-sys/build/bind_stripped.xml")
    let mut file =
        std::fs::File::open("/Volumes/src/usd-sys/build/bind.xml").expect("Unable to read file");

    // Read the output from cast xml
    let root: cs::CastXML = from_reader(file).expect("Unable to read xml file");

    // Transform it into an ast in C
    let output = Transform::apply(&root);

    // Dump the transformed c ast into headers and cpp files
    write_headers(&output);
    write_source(&output);
}
