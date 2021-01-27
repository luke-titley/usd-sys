//------------------------------------------------------------------------------
// vfx-rs 2021
//------------------------------------------------------------------------------
use serde::{Deserialize, Serialize};

use std::string::String;
use std::vec::Vec;
use std::sync::Arc;

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
pub struct Argument {
    pub name: Option<Name>,
    #[serde(rename = "type")]
    pub type_: IdRef,
    pub original_type: Option<IdRef>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<String>,
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
    pub static_: Option<String>,
    pub inline: Option<String>,
    #[serde(rename = "extern")]
    pub extern_: Option<String>,
    pub artificial: Option<String>,
    pub throw: Option<IdRefs>,
    pub mangled: Option<String>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
    pub attributes: Option<Attributes>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<String>,
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
    pub static_: Option<String>,
    pub inline: Option<String>,
    #[serde(rename = "extern")]
    pub extern_: Option<String>,
    pub artificial: Option<String>,
    pub throw: Option<IdRefs>,
    pub mangled: Option<String>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
    pub attributes: Option<Attributes>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<String>,

    // Added
    pub access: Access,
    pub explicit: Option<String>,
    pub const_: Option<String>,
    pub virtual_: Option<String>,
    pub pure_virtual: Option<String>,
    pub overrides: Option<IdRefs>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FunctionType {
    #[serde(rename = "IdRef", default)]
    pub arguments: Vec<IdRef>,

    pub id: Id,
    pub returns: IdRef,
    pub const_: Option<String>,
    pub volatile_: Option<String>,
    pub restrict_: Option<String>,
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Base {
    #[serde(rename = "type")]
    pub type_: IdRef,
    pub access: Access,
    #[serde(rename = "virtual")]
    pub virtual_: bool,
    pub offset: Option<String>,
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
    pub line: Option<String>,
    pub comment: Option<IdRef>,
    pub incomplete: Option<String>,
    pub abstract_: Option<String>,
    pub members: Option<IdRefs>,
    pub befriending: Option<IdRefs>,
    pub size: Option<String>,
    pub align: Option<String>,
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnumValue {
    name: Name,
    init: String,
    attributes: Option<Attributes>,
    deprecation: Option<Name>,
    annotation: Option<Name>,
}

#[derive(Debug, serde::Deserialize)]
pub enum Functions {
    Function(Function),
    OperatorFunction(Function),
}

#[derive(Debug, serde::Deserialize)]
pub enum Methods {
    Constructor(Method),
    Destructor(Method),
    Method(Method),
    OperatorMethod(Method),
    Converter(Method),
}

#[derive(Debug, serde::Deserialize)]
pub enum Types {
    Typedef {
        id: Id,
        name: Name,
        #[serde(rename = "type")]
        type_: IdRef,
        context: Option<IdRef>,
        access: Option<Access>,
        location: Option<String>,
        file: Option<String>,
        line: Option<String>,
        attributes: Option<Attributes>,
        comment: Option<IdRef>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
    },
    FundamentalType {
        id: Id,
        name: Name,
        size: Option<String>,
        align: Option<String>,
    },
    CvQualifiedType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        const_: Option<String>,
        volatile_: Option<String>,
        restrict_: Option<String>,
    },
    PointerType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        size: Option<String>,
        align: Option<String>,
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
        size: String,
        align: String,
    },
    ArrayType {
        id: Id,
        #[serde(rename = "type")]
        type_: IdRef,
        min: String,
        max: Option<String>,
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
        const_: Option<String>,
        #[serde(rename = "volatile")]
        volatile_: Option<String>,
        #[serde(rename = "restrict")]
        restrict_: Option<String>,
        attributes: Option<Attributes>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
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
        line: Option<String>,
        scoped: Option<String>,
        size: String,
        align: String,
        attributes: Option<Attributes>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
        comment: Option<IdRef>,
    },
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
        begin_line: String,
        begin_column: String,
        begin_offset: String,
        end_line: String,
        end_column: String,
        end_offset: String,
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
        line: Option<String>,
        #[serde(rename = "static")]
        static_: Option<String>,
        #[serde(rename = "extern")]
        extern_: Option<String>,
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
        bits: Option<String>,
        context: IdRef,
        access: Access,
        location: Option<String>,
        file: Option<String>,
        line: Option<String>,
        offset: String,
        mutable: Option<String>,
        attributes: Option<Attributes>,
        comment: Option<IdRef>,
        deprecation: Option<Name>,
        annotation: Option<Name>,
    },
    Functions(Functions),
    Methods(Methods),
    Type(Type),
    Unimplemented {
        id: Id,
        kind: Option<Name>,
        type_class: Option<Name>,
    },
}

#[derive(Debug, serde::Deserialize)]
pub struct AST {
    #[serde(rename = "$value")]
    pub items: Vec<Item>,
}
