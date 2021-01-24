

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Deserialize)]
pub type Token = String;

#[derive(Debug, Deserialize)]
pub struct IdRef(pub u64);
type IdRefs = Vec<IdRef>;


#[derive(Debug, Deserialize)]
pub struct Bool(pub bool);
#[derive(Debug, Deserialize)]
pub struct Name(pub Token);

#[derive(Debug, Deserialize)]
pub struct Empty();
#[derive(Debug, Deserialize)]
pub struct NameOrEmpty(pub Option<Name, Empty>);
#[derive(Debug, Deserialize)]
pub struct Expression(pub Token);
#[derive(Debug, Deserialize)]
pub struct Attributes(pub Token);

pub struct Id = u64;

#[derive(Debug, Deserialize)]
pub enum Access {
    Public,
    Protected,
    Private
}

/*
  <xs:attributeGroup name="abi">
    <xs:attribute name="size" type="xs:unsignedLong" />
    <xs:attribute name="align" type="xs:unsignedLong" />
  </xs:attributeGroup>

  <xs:attributeGroup name="optionalAbi">
    <xs:attribute name="size" type="xs:unsignedLong" use="optional" />
    <xs:attribute name="align" type="xs:unsignedLong" use="optional" />
  </xs:attributeGroup>

  <xs:attributeGroup name="location">
    <xs:attribute name="location" use="optional">
      <xs:simpleType>
        <xs:restriction base="xs:token">
          <xs:pattern value="f[0-9]+:[0-9]+" />
        </xs:restriction>
      </xs:simpleType>
    </xs:attribute>
    <xs:attribute name="file" type="xs:IDREF" use="optional" />
    <xs:attribute name="line" type="xs:unsignedLong" use="optional" />
  </xs:attributeGroup>
*/

#[derive(Debug, Deserialize)]
pub struct Argument {
  pub name Option<Name>,
  pub type: IdRef,
  pub original_type: Option<IdRef>,
  pub location: Option<String>,
  pub file: Option<String>,
  pub line: Option<u64>,
  pub default: Option<Expression>,
  pub attributes: Option<Attributes>,
  pub deprecation: Option<Name>,
  pub annotation: Option<name>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "Argument", default)]
    pub arguments: Vec<Argument>
    pub id: Id,
    pub name: NameOrEmpty,
    pub returns : Option < IdRef >,
    pub context : Option < IdRef >,
    pub static : Option < i32 >,
    pub inline : Option < i32 >,
    pub extern : Option <i32 >,
    pub artificial : Option < i32 >,
    pub throw : Option < IdRefs >,
    pub mangled : Option < String >,
    pub deprecation : Option < Name >,
    pub annotation : Option < Name >,
    pub attributes : Option < Attributes >,
    pub comment : Option < String >,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    #[serde(rename = "Argument", default)]
    pub arguments: Vec<Argument>
    pub id: Id,
    pub name: NameOrEmpty,
    pub returns : Option < IdRef >,
    pub context : Option < IdRef >,
    pub static : Option < i32 >,
    pub inline : Option < i32 >,
    pub extern : Option <i32 >,
    pub artificial : Option < i32 >,
    pub throw : Option < IdRefs >,
    pub mangled : Option < String >,
    pub deprecation : Option < Name >,
    pub annotation : Option < Name >,
    pub attributes : Option < Attributes >,
    pub comment : Option < String >,
    pub location: Option<String>,
    pub file: Option<String>,
    pub line: Option<u64>,

    // Added
    #[serde(rename = "Argument", default)]
    dec_access: Acces,
    #[serde(rename = "Argument", default)]
    dec_explicit: Option<u32>,
    #[serde(rename = "Argument", default)]
    dec_const: Option<u32>,
    #[serde(rename = "Argument", default)]
    dec_virtual: Option<u32>,
    #[serde(rename = "Argument", default)]
    dec_pure_virtual: Option<u32>,
    #[serde(rename = "Argument", default)]
    dec_overrides: Option<IdRefs>,
}

pub struct FunctionType {
    #[serde(rename = "IdRef", default)]
    pub arguments: Vec<IdRef>

    pub id: Id,
    pub returns: IdRef,
    pub const_: Option<u32>
    pub volatile_: Option<u32>
    pub restrict_: Option<u32>
    pub attributes: Option<Attributes>,
    pub deprecation: Option<Name>,
    pub annotation: Option<Name>
}

pub struct Base {
    pub dec_type: IdRef,
    pub dec_access: Access,
    pub dec_virtual: bool,
    pub offset: Option<u64>,
}

pub struct Record {
    #[serde(rename = "Basef", default)]
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
    pub size: option<u64>,
    pub align: option<u64>,
    pub attributes : Option <Attributes >,
    pub deprecation : Option < Name >,
    pub annotation : Option < Name >,
}

type Items = Vec<Item>;

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
    }
}

      <xs:element name="ElaboratedType">
        <xs:complexType>
          <xs:attribute name="id" type="xs:ID" />
          <xs:attribute name="type" type="xs:IDREF" />
        </xs:complexType>
      </xs:element>

      <xs:element name="FunctionType" type="FunctionType" />

      <xs:element name="MethodType">
        <xs:complexType>
          <xs:complexContent>
            <xs:extension base="FunctionType">
              <xs:attribute name="basetype" type="xs:IDREF" />
            </xs:extension>
          </xs:complexContent>
        </xs:complexType>
      </xs:element>

      <xs:element name="Unimplemented">
        <xs:complexType>
          <xs:attribute name="id" type="xs:ID" />
          <!-- kind is set for unimplemented declarations, type_class for types. -->
          <xs:attribute name="kind" type="name" use="optional" />
          <xs:attribute name="type_class" type="name" use="optional" />
        </xs:complexType>
      </xs:element>
    </xs:choice>
  </xs:complexType>

  <!-- castxml-output=1 output root. -->
  <xs:element name="CastXML">
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="Items">
          <xs:attribute name="format">
            <xs:simpleType>
              <xs:restriction base="xs:token">
                <xs:pattern value="1(\.[0-9]+)*" />
              </xs:restriction>
            </xs:simpleType>
          </xs:attribute>
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>

  <!-- castxml-gccxml output root. -->
  <xs:element name="GCC_XML">
    <xs:complexType>
      <xs:complexContent>
        <xs:extension base="Items">
          <xs:attribute name="version" type="xs:token" fixed="0.9.0" />
          <xs:attribute name="cvs_revision" type="xs:token" fixed="1.145" />
        </xs:extension>
      </xs:complexContent>
    </xs:complexType>
  </xs:element>
</xs:schema>
