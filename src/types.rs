use crate::common::{filters, BooleanLiteral, NumericLiteral, StringLiteral};
use askama::Template;
use displaythis::Display;
use from_variants::FromVariants;

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ inner_type }}[]", ext = "txt")]
pub struct ArrayType {
    pub inner_type: Box<PrimaryType>,
}

impl ArrayType {
    pub fn new(primary: PrimaryType) -> Self {
        ArrayType {
            inner_type: Box::new(primary),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "<{{ identifiers|join(\", \") }}>", ext = "txt")]
pub struct TypeParameters {
    pub identifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "[ {{ inner_types|join(\", \") }} ]", ext = "txt")]
pub struct TupleType {
    pub inner_types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
pub enum LiteralType {
    #[display("{0}")]
    StringLiteral(StringLiteral),
    #[display("{0}")]
    NumericLiteral(NumericLiteral),
    #[display("{0}")]
    BooleanLiteral(BooleanLiteral),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "{% match namespace %}{% when Some with (namespace) %}{{ namespace }}.{% when None %}{% endmatch %}{{- ident -}}",
    ext = "txt"
)]
pub struct TypeName {
    pub ident: String,
    pub namespace: Option<Box<TypeName>>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "< {{ types|join(\", \") }} >", ext = "txt")]
pub struct TypeArguments {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
pub enum TsType {
    #[display("{0}")]
    PrimaryType(PrimaryType),
    #[display("{0}")]
    UnionType(UnionType),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ name }}{{ args|display_opt }}", ext = "txt")]
pub struct TypeReference {
    pub name: TypeName,
    pub args: Option<TypeArguments>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ types|join(\" | \") }}", ext = "txt")]
pub struct UnionType {
    pub types: Vec<TsType>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{ {{- body|display_opt -}} }", ext = "txt")]
pub struct ObjectType {
    pub body: Option<TypeBody>,
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(source = "{{ members|join(\",\\n\") }}", ext = "txt")]
pub struct TypeBody {
    pub members: Vec<TypeMember>,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
pub enum TypeMember {
    #[display("{0}")]
    PropertySignature(PropertySignature),
}

#[derive(Debug, Clone, PartialEq, Template)]
#[template(
    source = "{{ name }}{% if optional %}?{% endif %}: {{ inner_type }}",
    ext = "txt"
)]
pub struct PropertySignature {
    pub name: PropertyName,
    pub optional: bool,
    pub inner_type: TsType,
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
pub enum PropertyName {
    #[display("{0}")]
    Identifier(String),
    #[display("{0}")]
    StringLiteral(StringLiteral),
}

#[derive(Debug, Clone, PartialEq, Display, FromVariants)]
pub enum PrimaryType {
    #[display("{0}")]
    Predefined(PredefinedType),
    #[display("{0}")]
    TypeReference(TypeReference),
    #[display("{0}")]
    ObjectType(ObjectType),
    #[display("{0}")]
    ArrayType(ArrayType),
    #[display("{0}")]
    TupleType(TupleType),
    #[display("{0}")]
    LiteralType(LiteralType),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PredefinedType {
    #[display("any")]
    Any,
    #[display("number")]
    Number,
    #[display("boolean")]
    Boolean,
    #[display("string")]
    String,
    #[display("unknown")]
    Unknown,
    #[display("null")]
    Null,
    #[display("never")]
    Never,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn display_primary_type() {
        assert_eq!(
            PrimaryType::Predefined(PredefinedType::Any).to_string(),
            "any"
        );

        assert_eq!(
            PrimaryType::TypeReference(TypeReference {
                args: None,
                name: TypeName {
                    ident: "MyType".to_string(),
                    namespace: None,
                }
            })
            .to_string(),
            "MyType"
        );
    }

    #[test]
    fn display_property_signature() {
        assert_eq!(
            PropertySignature {
                name: PropertyName::Identifier("test".to_string()),
                optional: false,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::String))
            }
            .to_string(),
            "test: string"
        );

        assert_eq!(
            PropertySignature {
                name: PropertyName::Identifier("test".to_string()),
                optional: true,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Number))
            }
            .to_string(),
            "test?: number"
        );

        assert_eq!(
            PropertySignature {
                name: PropertyName::StringLiteral("test".into()),
                optional: true,
                inner_type: TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Number))
            }
            .to_string(),
            "\"test\"?: number"
        );
    }

    #[test]
    fn display_type_body() {
        assert_eq!(
            TypeBody {
                members: vec![
                    TypeMember::PropertySignature(PropertySignature {
                        name: PropertyName::Identifier("test".into()),
                        optional: false,
                        inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                            PredefinedType::Number
                        ))
                    }),
                    TypeMember::PropertySignature(PropertySignature {
                        name: PropertyName::StringLiteral("test_other".into()),
                        optional: false,
                        inner_type: TsType::PrimaryType(PrimaryType::Predefined(
                            PredefinedType::Any
                        ))
                    }),
                ]
            }
            .to_string(),
            "test: number,\n\"test_other\": any",
        );
    }

    #[test]
    fn display_tuple_types() {
        assert_eq!(
            TupleType {
                inner_types: vec![
                    TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Any)),
                    TsType::PrimaryType(PrimaryType::Predefined(PredefinedType::Boolean)),
                ],
            }
            .to_string(),
            "[ any, boolean ]"
        );
    }

    #[test]
    fn display_array_type() {
        assert_eq!(
            ArrayType::new(PrimaryType::Predefined(PredefinedType::Any)).to_string(),
            "any[]"
        );
    }

    #[test]
    fn display_object_type() {
        assert_eq!(ObjectType { body: None }.to_string(), "{}",);
    }
}
