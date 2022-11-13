use crate::input::Input;
use proc_macro2::Span;

pub fn impl_filter_wrapper(
	input: &Input,
	struct_name: &str,
	span: &Span,
) -> proc_macro2::TokenStream {
	let struct_ident = syn::Ident::new(struct_name, *span);
	let builder_ident = syn::Ident::new((struct_name.to_owned() + "Builder").as_str(), *span);
	
	let impl_struct = impl_struct(input, &struct_ident);
	let impl_from_filter = impl_from_filter(input, &struct_ident, &builder_ident);
	let impl_from_struct = impl_from_struct(input, &struct_ident, &builder_ident);
	let impl_builder = impl_builder(input, &struct_ident, &builder_ident);
	
	quote::quote! {
		#impl_struct
		#impl_from_filter
		#impl_from_struct
		#impl_builder
	}
}

fn impl_struct(_input: &Input, struct_ident: &syn::Ident) -> proc_macro2::TokenStream {
	quote::quote! {
		pub struct #struct_ident {
			pub filter: Option<Filter>,
		}
	}
}

fn impl_from_struct(
	input: &Input,
	struct_ident: &syn::Ident,
	builder_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
	let ident = &input.ast.ident;
	
	quote::quote! {
		impl From<#ident> for #struct_ident {
			fn from(item: #ident) -> Self {
				#builder_ident::default()
					.set_filter(Filter::from(item))
					.build()
			}
		}
	}
}

fn impl_from_filter(
	_input: &Input,
	struct_ident: &syn::Ident,
	builder_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
	quote::quote! {
		impl From<Filter> for #struct_ident {
			fn from(filter: Filter) -> Self {
				#builder_ident::default()
					.set_filter(filter)
					.build()
			}
		}
	}
}

fn impl_builder(
	_input: &Input,
	struct_ident: &syn::Ident,
	builder_ident: &syn::Ident,
) -> proc_macro2::TokenStream {
	quote::quote! {
		#[derive(Default)]
		pub struct #builder_ident {
			filter: Option<Filter>,
		}
		
		impl #builder_ident {
			pub fn set_filter(mut self, filter: Filter) -> #builder_ident {
				self.filter = Some(filter); self
			}
			
			pub fn build(self) -> #struct_ident {
				#struct_ident { filter: self.filter }
			}
		}
	}
}
