extern crate core;

mod impl_filter;
mod impl_filter_wrapper;
mod impl_select;
mod input;
mod worm_table;

use input::sql_type::SqlType;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;
use syn::DeriveInput;

/// Macro definition for worm_table
#[proc_macro_derive(WormTable, attributes(integer, varchar, text))]
#[proc_macro_error]
pub fn worm_table(input: TokenStream) -> proc_macro::TokenStream {
	let derive_input = parse_macro_input!(input as DeriveInput);
	worm_table::derive(&derive_input).into()
}
