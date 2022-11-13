use proc_macro_error::abort;
use std::borrow::Cow;

pub enum SqlType {
    Integer,
    Varchar(u64),
    Text,
}

impl SqlType {
    pub fn from_type(ty: &syn::Type, attr: &str, size: Option<u64>) -> SqlType {
        if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
            match (ty, attr, size) {
                (_, "varchar", Some(size)) if path.is_ident("String") => SqlType::Varchar(size),
                (_, "varchar", _) if path.is_ident("String") => abort!(ty, "Unknown size"),
                (_, "varchar", _) => abort!(path, "Unknown type for attribute"),
                (_, "text", Some(_)) => abort!(ty, "Size for type not supported"),
                (_, "text", None) => SqlType::Text,
                (_, "integer", Some(_)) => abort!(path, "Size for type not supported"),
                (_, "integer", None) if path.is_ident("u64") => SqlType::Integer,
                (_, "integer", None) if path.is_ident("u32") => SqlType::Integer,
                (_, "integer", None) if path.is_ident("i64") => SqlType::Integer,
                (_, "integer", None) if path.is_ident("i32") => SqlType::Integer,
                _ => abort!(ty, "Unknown type"),
            }
        } else {
            abort!(ty, "Unknown type")
        }
    }

    pub fn to_string(&self) -> Cow<str> {
        match self {
            SqlType::Integer => Cow::from("INTEGER"),
            SqlType::Text => Cow::from("TEXT"),
            SqlType::Varchar(size) => Cow::from(format!("VARCHAR({})", size)),
        }
    }
}
