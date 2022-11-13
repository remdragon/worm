use crate::impl_filter::impl_filter;
use crate::impl_filter_wrapper::impl_filter_wrapper;
use crate::input::Input;
use crate::SqlType;

pub fn derive(derive_input: &syn::DeriveInput) -> proc_macro2::TokenStream {
	let input = Input::from_syn(derive_input);
	input.validate();
	
	let impl_enum = impl_enum(&input);
	let impl_create_table = impl_create_table(&input);
	let impl_delete_table = impl_delete_table(&input);
	let impl_insert = impl_insert(&input);
	let impl_update = impl_update(&input);
	let filter_struct = impl_filter(&input);
	let select_struct = crate::impl_select::impl_select(&input);
	let impl_select_one_wrapper = impl_filter_wrapper(&input, "SelectOne", &input.name.span());
	let impl_count_wrapper = impl_filter_wrapper(&input, "Count", &input.name.span());
	let impl_delete_wrapper = impl_filter_wrapper(&input, "Delete", &input.name.span());
	let impl_struct_table = impl_struct_table(&input);
	let impl_constructor = impl_constructor(&input);
	let impl_select_all = impl_select_all(&input);
	let impl_select_one = impl_select_one(&input);
	let impl_select = impl_select(&input);
	let impl_count_all = impl_count_all(&input);
	let impl_count = impl_count(&input);
	let impl_delete_all = impl_delete_all(&input);
	let impl_delete = impl_delete(&input);
	
	quote::quote! {
		#impl_create_table
		#impl_delete_table
		#impl_insert
		#impl_update
		#impl_enum
		#impl_count_wrapper
		#impl_count
		#impl_count_all
		#impl_constructor
		#impl_struct_table
		#filter_struct
		#select_struct
		#impl_select_one_wrapper
		#impl_select_one
		#impl_select
		#impl_select_all
		#impl_delete_wrapper
		#impl_delete
		#impl_delete_all
	}
}

fn impl_enum(_input: &Input) -> proc_macro2::TokenStream {
	let doc = "This enum provides an identification of the different SQL wrapper supported";
	
	quote::quote! {
	  #[doc = #doc]
	  enum SqlConnection<'a> {
			Rusqlite(&'a rusqlite::Connection),
	  }
	}
}

fn impl_struct_table(input: &Input) -> proc_macro2::TokenStream {
	let name_table = &input.name_table;
	let doc = format!(
		r#"
The struct {} wraps around the connection to the SQL database - specified as part of the constructor -
and manages the interaction between the software and the database
"#,
		name_table
	);
	
	quote::quote! {
		#[doc = #doc]
		pub struct #name_table<'a> {
			connection: SqlConnection<'a>,
		}
	}
}

fn impl_constructor(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	let doc = format!(
		r#"
Construct a SQL connector to manipulate struct of type {} to an SQLite database using the rusqlite wrapper
"#,
		input.name
	);
	
	quote::quote! {
		#[doc = #doc]
		impl #name {
			pub fn from_rusqlite<'a>(conn: &'a rusqlite::Connection) -> #name_table<'a> {
				#name_table { connection: SqlConnection::Rusqlite(conn) }
			}
		}
	}
}

fn impl_delete_all(input: &Input) -> proc_macro2::TokenStream {
	let name = &input.name;
	let name_table = &input.name_table;
	
	quote::quote! {
		impl #name {
			pub fn delete_all() -> String {
				#name::delete(DeleteBuilder::default().build())
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn delete_all(&self) -> Result<(), Box<dyn std::error::Error>> {
				self.delete(DeleteBuilder::default().build())
			}
		}
	}
}

fn impl_delete(input: &Input) -> proc_macro2::TokenStream {
	let name = &input.name;
	let name_table = &input.name_table;
	let statement = format!("DELETE FROM {}", input.get_table_name());
	
	quote::quote! {
		impl #name {
			pub fn delete(delete: Delete) -> String {
				let mut statement = #statement.to_string();
				if let Some(filter) = delete.filter {
					statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
				}
				statement
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn delete(&self, delete: Delete) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						let statement = #name::delete(delete);
						conn.execute(statement.as_str(), ())?;
						Ok(())
					}
				}
			}
		}
	}
}

fn impl_update(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	let fields_named = &input.fields();
	
	let statement_to = format!(
		"UPDATE {} SET {} WHERE {}",
		input.get_table_name(),
		collect_join(
			fields_named.iter().enumerate().map(|(i, f)| format!(
				"{} = ?{}",
				f.ident,
				i + 1 + fields_named.len()
			)),
			", "
		),
		collect_join(
			fields_named
				.iter()
				.enumerate()
				.map(|(i, f)| format!("{} = ?{}", f.ident, i + 1)),
			" AND "
		)
	);
	
	let primary_keys = fields_named.iter().filter(|f| f.primary_key).count();
	let statement_by_id = format!(
		"UPDATE {} SET {} WHERE {}",
		input.get_table_name(),
		collect_join(
			fields_named.iter().enumerate().map(|(i, f)| format!(
				"{} = ?{}",
				f.ident,
				i + 1 + primary_keys
			)),
			", "
		),
		collect_join(
			fields_named
				.iter()
				.enumerate()
				.filter(|(_, f)| f.primary_key)
				.map(|(i, f)| format!("{} = ?{}", f.ident, i + 1)),
			" AND "
		)
	);
	
	let primary_key_parameters: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident).collect();
	let parameters: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident).collect();
	
	quote::quote! {
		impl #name {
			pub fn update_by_id() -> String {
				String::from(#statement_by_id)
			}
			
			pub fn update_to() -> String {
				String::from(#statement_to)
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn update_by_id(&self, obj: &#name) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						conn.execute(#statement_by_id, ( #( &obj.#primary_key_parameters ),* , #( &obj.#parameters ),* ))?;
						Ok(())
					}
				}
			}
			
			pub fn update_to(&self, from: &#name, to: &#name) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						conn.execute(#statement_to, ( #( &from.#parameters ),* , #( &to.#parameters ),* ))?;
						Ok(())
					}
				}
			}
		}
	}
}

fn impl_select_all(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	
	quote::quote! {
		impl #name {
			pub fn select_all() -> String {
				#name::select(SelectBuilder::default().build())
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn select_all(&self) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
				self.select(SelectBuilder::default().build())
			}
		}
	}
}

fn impl_select_one(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	
	quote::quote! {
		impl<'a> #name_table<'a> {
			pub fn select_one(&self, select_one: SelectOne) -> Result<Option<#name>, Box<dyn std::error::Error>> {
				let r = self.select(select_one.into())?;
				Ok(r.into_iter().nth(0))
			}
		}
	}
}

fn impl_select(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	let fields_named = &input.fields();
	
	let statement = format!(
		"SELECT {} FROM {}",
		collect_join(fields_named.iter().map(|f| format!("{}", f.ident)), ", "),
		input.get_table_name()
	);
	let fields: Vec<&syn::Ident> = fields_named.iter().map(|f| f.ident).collect();
	let fields_assignment: Vec<proc_macro2::TokenStream> = fields_named
		.iter()
		.enumerate()
		.map(|(i, _)| quote::quote! { r.get(#i)? })
		.collect();
	
	quote::quote! {
		impl #name {
			pub fn select(select: Select) -> String {
				let mut statement = #statement.to_string();
				if let Some(filter) = select.filter {
					statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
				}
				if let Some(limit) = select.limit {
					statement += format!(" LIMIT {}", limit).as_str();
				}
				if let Some(offset) = select.offset {
					statement += format!(" ORDER BY ( SELECT NULL ) OFFSET {}", offset).as_str();
				}
				statement
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn select(&self, select: Select) -> Result<Vec<#name>, Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						let statement = #name::select(select);
						let mut s = conn.prepare(statement.as_str())?;
						let r = s.query_map([], |r| Ok( #name { #( #fields : #fields_assignment ),* } ) )?
							.collect::<Result<Vec<#name>, rusqlite::Error>>()?;
						Ok(r)
					}
				}
			}
		}
	}
}

fn impl_count_all(input: &Input) -> proc_macro2::TokenStream {
	let name = &input.name;
	let name_table = &input.name_table;
	quote::quote! {
		impl #name {
			pub fn count_all_statement() -> String {
				#name::count_statement(CountBuilder::default().build())
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn count_all(&self) -> Result<usize, Box<dyn std::error::Error>> {
			  self.count(CountBuilder::default().build())
			}
		}
	}
}

fn impl_count(input: &Input) -> proc_macro2::TokenStream {
	let name = &input.name;
	let name_table = &input.name_table;
	let fields_named = &input.fields();
	
	let statement = format!(
		"SELECT COUNT( {} ) FROM {}",
		fields_named.iter().next().unwrap().ident,
		input.get_table_name()
	);
	
	quote::quote! {
		impl #name {
			pub fn count_statement(count: Count) -> String {
				let mut statement = #statement.to_string();
				if let Some(filter) = count.filter {
					statement += format!(" WHERE {}", Filter::to_condition(&filter)).as_str();
				}
				statement
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn count(&self, count: Count) -> Result<usize, Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						let statement = #name::count_statement(count);
						let mut s = conn.prepare(statement.as_str())?;
						let r = s.query_map([], |r| r.get(0))?.collect::<Result<Vec<usize>, rusqlite::Error>>()?;
						if r.len() == 1 {
							Ok(r[0])
						} else {
							Err(format!("SELECT COUNT result is expected to have length of 1. Current result is {:?}", r).into())
						}
					}
				}
			}
		}
	}
}

fn impl_insert(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	
	let statement = format!(
		"INSERT INTO {} ({}) VALUES ({})",
		input.get_table_name(),
		collect_join(input.fields_iter().map(|f| format!("{}", f.ident)), ", "),
		collect_join(
			input
				.fields_iter()
				.enumerate()
				.map(|(i, _)| format!("?{}", i + 1)),
			", "
		),
	);
	
	let parameters: Vec<&syn::Ident> = input.fields_iter().map(|f| f.ident).collect();
	
	quote::quote! {
		impl #name {
			pub fn insert() -> String {
				String::from(#statement)
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn insert(&self, i: &#name) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						conn.execute(#statement, ( #( &i.#parameters ),* ))?;
						Ok(())
					}
				}
			}
		}
	}
}

fn impl_delete_table(input: &Input) -> proc_macro2::TokenStream {
	let name = &input.name;
	let name_table = &input.name_table;
	let statement = format!("DROP TABLE {}", input.get_table_name());
	
	quote::quote! {
		impl #name {
			pub fn delete_table() -> String {
				String::from(#statement)
			}
		}
		
		impl<'a> #name_table<'a> {
			pub fn delete_table(&self) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						conn.execute(#statement, ())?;
						Ok(())
					},
				}
			}
		}
	}
}

fn impl_create_table(input: &Input) -> proc_macro2::TokenStream {
	let name = input.name;
	let name_table = &input.name_table;
	
	let fields_statement = collect_join(
		input.fields_iter().map(|field| {
			let sql_type = SqlType::to_string(&field.sql_type);
			
			let ident = field.ident.to_string();
			let mut attributes = vec![ident.as_str(), sql_type.as_ref()];
			
			if field.primary_key {
				attributes.push("NOT NULL PRIMARY KEY");
			} else if let Some(nullable) = field.nullable {
				if nullable {
					attributes.push("NULL");
				} else {
					attributes.push("NOT NULL");
				}
			}
			if field.unique {
				attributes.push("UNIQUE");
			}
			
			collect_join_str(attributes.into_iter().filter(|str| !str.is_empty()), " ")
		}),
		", ",
	);
	
	let statement = format!(
		"CREATE TABLE IF NOT EXISTS {} ( {} )",
		input.get_table_name(),
		fields_statement
	);
	
	quote::quote! {
		impl #name {
			pub fn create_table() -> String {
				String::from(#statement)
			}
		}
		
		impl<'a> #name_table <'a> {
			pub fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> {
				match self.connection {
					SqlConnection::Rusqlite(conn) => {
						conn.execute(#statement, ())?;
						Ok(())
					}
				}
			}
		}
	}
}

fn collect_join_str<'a, T: Iterator<Item = &'a str>>(iter: T, separator: &str) -> String {
	iter.collect::<Vec<&str>>().join(separator)
}

fn collect_join<T: Iterator<Item = String>>(iter: T, separator: &str) -> String {
	iter.collect::<Vec<String>>().join(separator)
}
