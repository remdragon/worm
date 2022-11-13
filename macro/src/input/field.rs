use crate::input::parse_utils;
use crate::SqlType;
use proc_macro_error::abort;
use syn::{DeriveInput, Fields};

pub struct Field<'a> {
	pub field: &'a syn::Field,
	pub ident: &'a syn::Ident,
	pub ty: &'a syn::Type,
	pub sql_type: SqlType,
	pub attribute: String,
	pub nullable: Option<bool>,
	pub primary_key: bool,
	pub unique: bool,
	pub size: Option<u64>,
}

impl<'a> Field<'a> {
	pub fn from_derive_input(ast: &'a syn::DeriveInput) -> Vec<Field> {
		let mut result = vec![];
		if let Some(syn::Fields::Named(fields_named)) = Self::get_fields(ast) {
			for field in fields_named.named.iter() {
				if let Some(f) = Self::from_field(field) {
					result.push(f)
				};
			}
		}
		result
	}
	
	fn from_field(field: &syn::Field) -> Option<Field> {
		if let Some(ident) = &field.ident {
			let mut attribute = String::from("");
			let mut nullable = None;
			let mut primary_key = false;
			let mut unique = false;
			let mut size = None;
			
			for attr in &field.attrs {
				match attr.parse_meta().unwrap() {
					syn::Meta::List(syn::MetaList { path, nested, .. }) => {
						for meta in nested.iter() {
							match meta {
								syn::NestedMeta::Meta(syn::Meta::NameValue(
									syn::MetaNameValue { path, lit, .. },
								)) => {
									if let Some(attr) = path.get_ident() {
										match attr.to_string().to_lowercase().as_str() {
											"null" => {
												nullable = Some(parse_utils::parse_bool_lit(lit))
											}
											"size" => {
												size = Some(parse_utils::parse_integer_lit(lit))
											}
											"unique" => unique = parse_utils::parse_bool_lit(lit),
											"primary" => {
												primary_key = parse_utils::parse_bool_lit(lit)
											}
											_ => {
												abort!(attr, "Unknown attribute");
											}
										}
									}
								}
								_ => {
									abort!(attr, "malformed attribute syntax");
								}
							}
						}
						if let Some(i) = path.get_ident() {
							attribute = i.to_string();
						}
					}
					_ => {
						abort!(attr, "malformed attribute syntax");
					}
				};
			}
			let sql_type = SqlType::from_type(&field.ty, &attribute, size);
			
			Some(Field {
				field,
				ident,
				ty: &field.ty,
				attribute,
				sql_type,
				nullable,
				unique,
				primary_key,
				size,
			})
		} else {
			None
		}
	}
	
	pub fn get_fields(ast: &DeriveInput) -> Option<&Fields> {
		if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
			Some(fields)
		} else {
			None
		}
	}
}
