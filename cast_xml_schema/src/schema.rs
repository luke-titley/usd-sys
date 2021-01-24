//------------------------------------------------------------------------------
// rust-vfx 2020
//------------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

use std::string::String;
use std::vec::Vec;

pub type Token = String;
pub type IdRef = u64;
pub type IdRefs = Vec<IdRef>;

pub type Bool = bool;
pub type Name = Token;

pub type NameOrEmpty = Option<Name>;
pub type Expression = Token;
pub type Attributes = Token;

pub type Id = u64;

#[derive(Debug, serde::Deserialize)]
pub enum Access {
    Public,
    Protected,
    Private,
}

#[derive(Debug, serde::Deserialize)]
pub struct Argument {
    pub name: Option<Name>,
    pub type_: IdRef,
    pub original_type: Option<IdRef>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,
    pub default: Option<Expression>,
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "Argument", default)]
    pub arguments: Vec<Argument>,
    pub id: Id,
    pub name: NameOrEmpty,
    pub returns: Option<IdRef>,
    pub context: Option<IdRef>,
    pub static_: Option<i32>,
    pub inline_: Option<i32>,
    pub extern_: Option<i32>,
    pub artificial: Option<i32>,
    pub throw: Option<IdRefs>,
    pub mangled: Option<String>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
    pub attributes: Option<Attributes>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    #[serde(rename = "Argument", default)]
    pub arguments: Vec<Argument>,
    pub id: Id,
    pub name: NameOrEmpty,
    pub returns: Option<IdRef>,
    pub context: Option<IdRef>,
    pub static_: Option<i32>,
    pub inline_: Option<i32>,
    pub extern_: Option<i32>,
    pub artificial: Option<i32>,
    pub throw: Option<IdRefs>,
    pub mangled: Option<String>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
    pub attributes: Option<Attributes>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,

    // Added
    access: Access,
    explicit: Option<u32>,
    const_: Option<u32>,
    virtual_: Option<u32>,
    pure_virtual: Option<u32>,
    overrides: Option<IdRefs>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionType {
    #[serde(rename = "IdRef", default)]
    pub arguments: Vec<IdRef>,

    pub id: Id,
    pub returns: IdRef,
    pub const_: Option<u32>,
    pub volatile_: Option<u32>,
    pub restrict_: Option<u32>,
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Base {
    pub dec_type: IdRef,
    pub dec_access: Access,
    pub dec_virtual: bool,
    pub offset: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    #[serde(rename = "Base", default)]
    pub base: Vec<Base>,
    pub id : Option < Id >,
    pub name : Option < NameOrEmpty >,
    pub context : Option < IdRef >,
    pub access : Option < Access >,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,
    pub comment : Option < IdRef >,
    pub incomplete : Option < i32 >,
    pub abstract_ : Option < i32 >,
    pub members : Option < IdRefs >,
    pub befriending : Option < IdRefs >,
    pub size: Option<u64>,
    pub align: Option<u64>,
    pub attributes : Option <Attributes >,
    pub deprecation : Option < Name >,
    pub annotation : Option < Name >,
}

/*
pub struct EnumValue {
    name: Name,
    init: u64,
    attributes: Option<Attributes>,
    deprecation: Option<Name>,
    annotation: Option<Name>,
}

pub enum Item {
    File {
        id: Id,
        name: Name,
    },
    Namespace {
        id: Id,
        name: Option<Name>,
        context: Option<IdRef>
        members: Option<IdRefs>
        name: Option<IdRef>
    },
    Comment {
        id: Id,
        attached: IdRef,
        file: IdRef,
        begin_line: u32,
        begin_column: u32,
        begin_offset: u32,
        end_line: u32,
        end_column: u32,
        end_offset: u32,
    }
    Struct(Record),
    Union(Record),
    Class(Record),
    Enumeration {
        #[serde(rename = "EnumValue", default)]
        enum_value : Vec<EnumValue>
          id: Id,
          name: NameOrEmpty,
          type_: IdRef,
          context: Option<IdRef>,
          access: Option<Access>,
          location: Option<String>,
          file: Option<String>,
          line: Option<u64>,
          scoped: Option<u32>,
          size: u64,
          align: u64,
          attributes: Option<Attributes>,
          deprecation: Option<Name>,
          annotation: Option<Name>,
          comment : Option < IdRef >,
    },
    Variable {
          id: Id,
          name: Name,
          type_: IdRef,
          init: Option<Expression>,
          context: Option<IdRef>,
          access: Option<Access>,
          location: Option<String>,
          file: Option<String>,
          line: Option<u64>,
          static_: Option<u32>
          extern_: Option<u32>
          mangled: String,
          attributes: Option<Attributes>
          comment: Option<IdRef>
          deprecation: Option<Name>
          annotation: Option<Name>
    },
    Field {
          id: Id,
          name: NameOrEmpty,
          type_: IdRef,
          bits: Option<u32>,
          context: IdRef,
          access: Access,
          <xs:attributeGroup ref="location" />
          offset: u64,
          mutable: Option<i32>,
          attributes: Option<Attributes>
          comment: Option<IdRef>,
          deprecation: Option<Name>,
          annotation: Option<Name>,
    },
    Function(Function),
    OperatorFunction(Function),
    Constructor(Method),
    Destructor(Method),
    Method(Method),
    OperatorMethod(Method),
    Converter(Method),
    Typedef {
      id: Id,
      name: Name,
      type_: IdRef,
      context: Option<IdRef>,
      access: Option<Access>,
      location: Option<String>,
      file: Option<String>,
      line: Option<u64>,
      attributes: Option<Attributes>,
      comment: Option<IdRef>,
      deprecation: Option<Name>,
      annotation: Option<Name>,
    },
    FundamentalType {
        id: Id,
        name: Name,
        size: Option<u64>,
        align: Option<u64>,
    },
    CvQualifiedType {
          id: Id,
          type_: IdRef,
          const_: Option<i32>,
          volatile_: Option<i32>,
          restrict_: Option<i32>,
    },
    PointerType {
          id: Id,
          type_: IdRef,
          size: Option<u64>,
          align: Option<u64>,
    }
    OffsetType {
          id: Id,
          basetype_: IdRef,
          type_: IdRef,
    }
    ReferenceType {
          id: Id,
          type_: IdRef,
          size: u64,
          align: u64,
    },
    ArrayType {
          id: Id,
          type_: IdRef,
          min: u64,
          max: Option<u64>,
    },
    ElaboratedType {
          id: Id,
          type_: IdRef,
    },
    FunctionType(FunctionType)
    MethodType {
        basetype: IdRef,
        #[serde(rename = "IdRef", default)]
        arguments: Vec<IdRef>

        id: Id,
        returns: IdRef,
        const_: Option<u32>
        volatile_: Option<u32>
        restrict_: Option<u32>
        attributes: Option<Attributes>,
        deprecation: Option<Name>,
        annotation: Option<Name>
    }
    UnImplemented {
        id: Id,
        kind: Option<Name>,
        type_class: Option<Name>,
    }
}

struct CastXML {
    items: Vec<Item>,
    format: String,
}

struct GCC_XML {
    #[serde(rename = "Item", default)]
    items: Vec<Item>,
    version: String,
    cvs_revision: String,
}
*/
