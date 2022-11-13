use crate::input::Input;

pub fn impl_select(input: &Input) -> proc_macro2::TokenStream {
    let impl_struct = impl_struct(input);
    let impl_from = impl_from(input);
    let impl_builder = impl_builder(input);

    quote::quote! {
        #impl_struct
        #impl_from
        #impl_builder
    }
}

/*
 *
 */
fn impl_struct(_input: &Input) -> proc_macro2::TokenStream {
    quote::quote! {
        pub struct Select {
            pub filter: Option<Filter>,
            pub limit: Option<usize>,
            pub offset: Option<usize>,
        }
    }
}

fn impl_from(_input: &Input) -> proc_macro2::TokenStream {
    quote::quote! {
        impl From<Filter> for Select {
            fn from(filter: Filter) -> Self {
                SelectBuilder::default()
                    .set_filter(filter)
                    .build()
            }
        }

        impl From<SelectOne> for Select {
            fn from(select_one: SelectOne) -> Self {
                let mut b = SelectBuilder::default();
                if let Some(filter) = select_one.filter { b = b.set_filter(filter); }
                b.set_limit(1)
                    .build()
            }
        }
    }
}

fn impl_builder(_input: &Input) -> proc_macro2::TokenStream {
    quote::quote! {
        #[derive(Default)]
        pub struct SelectBuilder {
            filter: Option<Filter>,
            limit: Option<usize>,
            offset: Option<usize>,
        }

        impl SelectBuilder {
            pub fn set_filter(mut self, filter: Filter) -> SelectBuilder {
                self.filter = Some(filter); self
            }

            pub fn set_limit(mut self, limit: usize) -> SelectBuilder {
                self.limit = Some(limit); self
            }

            pub fn set_offset(mut self, offset: usize) -> SelectBuilder {
                self.offset = Some(offset); self
            }

            pub fn build(self) -> Select {
                Select { filter: self.filter, limit: self.limit, offset: self.offset }
            }
        }
    }
}
