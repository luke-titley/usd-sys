//------------------------------------------------------------------------------
// rust-vfx 2020
//------------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

use std::string::String;
use std::vec::Vec;

pub type Token = String;
pub type IdRef = String;
pub type IdRefs = String;

pub type Bool = bool;
pub type Name = Token;

pub type NameOrEmpty = Option<Name>;
pub type Expression = Token;
pub type Attributes = String;

pub type Id = String;

#[derive(Debug, serde::Deserialize)]
pub enum Access {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "protected")]
    Protected,
    #[serde(rename = "private")]
    Private,
}

#[derive(Debug, serde::Deserialize)]
pub struct Argument {
    pub name: Option<Name>,
    #[serde(rename = "type")]
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
    #[serde(rename = "static")]
    pub static_: Option<i32>,
    pub inline: Option<i32>,
    #[serde(rename = "extern")]
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
    #[serde(rename = "static")]
    pub static_: Option<i32>,
    pub inline: Option<i32>,
    #[serde(rename = "extern")]
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
    pub access: Access,
    pub explicit: Option<u32>,
    pub const_: Option<u32>,
    pub virtual_: Option<u32>,
    pub pure_virtual: Option<u32>,
    pub overrides: Option<IdRefs>,
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
    pub id: Id,
    pub name: Option<NameOrEmpty>,
    pub context: Option<IdRef>,
    pub access: Option<Access>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,
    pub comment: Option<IdRef>,
    pub incomplete: Option<i32>,
    pub abstract_: Option<i32>,
    pub members: Option<IdRefs>,
    pub befriending: Option<IdRefs>,
    pub size: Option<u64>,
    pub align: Option<u64>,
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnumValue {
    name: Name,
    init: u64,
    attributes: Option<Attributes>,
    deprecation: Option<Name>,
    annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub enum Item {
    File {
        id: Id,
        name: Name,
    },

    Namespace {
        id: Id,
        name: Option<Name>,
        context: Option<IdRef>,
        members: Option<IdRefs>,
        comment: Option<IdRef>,
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
    },
    Struct(Record),
    Union(Record),
    Class(Record),
    Enumeration {
        #[serde(rename = "EnumValue", default)]
        enum_value: Vec<EnumValue>,
        id: Id,
        name: NameOrEmpty,
        #[serde(rename = "type")]
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
        comment: Option<IdRef>,
    },
    Variable {
        id: Id,
        name: Name,
        #[serde(rename = "type")]
        type_: IdRef,
        init: Option<Expression>,
        context: Option<IdRef>,
        access: Option<Access>,
        location: Option<String>,
        file: Option<String>,
        line: Option<u64>,
        #[serde(rename = "static")]
        static_: Option<u32>,
        #[serde(rename = "extern")]
        extern_: Option<u32>,
        mangled: String,
        attributes: Option<Attributes>,
        comment: Option<IdRef>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
    },
    Field {
        id: Id,
        name: NameOrEmpty,
        #[serde(rename = "type")]
        type_: IdRef,
        bits: Option<u32>,
        context: IdRef,
        access: Access,
        location: Option<String>,
        file: Option<String>,
        line: Option<u64>,
        offset: u64,
        mutable: Option<i32>,
        attributes: Option<Attributes>,
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
        #[serde(rename = "type")]
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
        #[serde(rename = "type")]
        type_: IdRef,
        const_: Option<i32>,
        volatile_: Option<i32>,
        restrict_: Option<i32>,
    },
    PointerType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        size: Option<u64>,
        align: Option<u64>,
    },
    OffsetType {
        id: Id,
        basetype_: IdRef,
        #[serde(rename = "type")]
        type_: IdRef,
    },
    ReferenceType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        size: u64,
        align: u64,
    },
    ArrayType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        min: u64,
        max: Option<u64>,
    },
    ElaboratedType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
    },
    FunctionType(FunctionType),
    MethodType {
        basetype: IdRef,
        #[serde(rename = "$value")]
        arguments: Vec<IdRef>,

        id: Id,
        returns: IdRef,
        #[serde(rename = "const")]
        const_: Option<u32>,
        #[serde(rename = "volatile")]
        volatile_: Option<u32>,
        #[serde(rename = "restrict")]
        restrict_: Option<u32>,
        attributes: Option<Attributes>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
    },
    UnImplemented {
        id: Id,
        kind: Option<Name>,
        type_class: Option<Name>,
    },
}

#[derive(Debug, serde::Deserialize)]
pub struct CastXML {
    #[serde(rename = "$value")]
    pub items: Vec<Item>,
    pub format: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GCC_XML {
    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
    pub version: String,
    pub cvs_revision: String,
}
