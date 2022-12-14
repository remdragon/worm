use crate::input::field::Field;
use proc_macro_error::abort;
use std::slice::Iter;

pub mod field;
mod parse_utils;
pub mod sql_type;

pub struct Input<'a> {
	pub ast: &'a syn::DeriveInput,
	fields: Vec<Field<'a>>,
	pub name: &'a syn::Ident,
	pub name_table: syn::Ident,
}

impl<'a> Input<'a> {
	pub fn from_syn(ast: &'a syn::DeriveInput) -> Input {
		let name = &ast.ident;
		let name_table = syn::Ident::new(format!("{}Table", name).as_str(), name.span());
		let fields = Field::from_derive_input(ast);
		
		Input {
			ast,
			name,
			name_table,
			fields,
		}
	}
	
	pub fn fields_iter(&self) -> Iter<Field> {
		self.fields.iter()
	}
	
	pub fn fields(&self) -> &Vec<Field> {
		&self.fields
	}
	
	pub fn validate(&'a self) {
		// validate data type
		match &self.ast.data {
			syn::Data::Struct(_) => {}
			syn::Data::Enum(_) => {
				abort!(&self.ast, "worm::Table is not supported for enum");
			}
			syn::Data::Union(_) => {
				abort!(&self.ast, "worm::Table is not supported for union");
			}
		};
		
		// validate the type of fields
		match Field::get_fields(self.ast).expect("Unable to retrieve fields") {
			field @ syn::Fields::Named(fields_named) => {
				if fields_named.named.is_empty() {
					abort!(field, "worm::Table does not support struct with no fields");
				}
			}
			field @ syn::Fields::Unnamed(_) => {
				abort!(field, "worm::Table does not support unnamed field");
			}
			field @ syn::Fields::Unit => {
				abort!(field, "worm::Table does not support unit");
			}
		}
	}
	
	pub fn get_table_name(&'a self) -> String {
		format!("{}", self.name)
	}
}
