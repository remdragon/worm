use crate::input::field::Field;
use crate::input::Input;
use crate::SqlType;
use convert_case::Casing;
use proc_macro2::TokenStream;

type FnStrToIdent = Box<dyn Fn(&str) -> syn::Ident>;

pub fn impl_filter(input: &Input) -> proc_macro2::TokenStream {
    let doc = r#"
Provides ability to nominate the filtering of results as part of the database query, aka WHERE in SQL queries
"#;

    let gt = |na: &str| {
        syn::Ident::new(
            &format!("{}GreaterThan", na),
            proc_macro2::Span::call_site(),
        )
    };
    let ge = |na: &str| {
        syn::Ident::new(
            &format!("{}GreaterEqualThan", na),
            proc_macro2::Span::call_site(),
        )
    };
    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());
    let le = |na: &str| {
        syn::Ident::new(
            &format!("{}LowerEqualThan", na),
            proc_macro2::Span::call_site(),
        )
    };
    let lt =
        |na: &str| syn::Ident::new(&format!("{}LowerThan", na), proc_macro2::Span::call_site());

    let fields = &input
        .fields_iter()
        .map(|field| {
            (
                field,
                format!("{}", field.ident).to_case(convert_case::Case::Pascal),
                format!("{}", field.ident),
            )
        })
        .collect::<Vec<(&Field, String, String)>>();

    let operator: Vec<FnStrToIdent> = vec![
        Box::new(gt),
        Box::new(ge),
        Box::new(eq),
        Box::new(le),
        Box::new(lt),
    ];
    let q_enum_types = operator
        .iter()
        .flat_map(|func| {
            fields.iter().map(|(field, name, _name_orig)| {
                let ident = func(name.as_str());
                let ty = field.ty;
                quote::quote! { #ident(#ty) }
            })
        })
        .collect::<Vec<TokenStream>>();
    let q_enum = quote::quote! {
        #[doc = #doc]
        #[derive(Clone)]
        pub enum Filter {
            And(Box<Filter>, Box<Filter>),
            Or(Box<Filter>, Box<Filter>),
            #( #q_enum_types ),*
        }
    };

    let operators: Vec<(FnStrToIdent, &str)> = vec![
        (Box::new(gt), ">"),
        (Box::new(ge), ">="),
        (Box::new(eq), "="),
        (Box::new(le), "<="),
        (Box::new(lt), "<"),
    ];
    let q_filter_matcher = operators
       .iter()
       .flat_map(|(func, op)| {
           fields.iter()
             .map(move |(field, name, name_orig)| {
                let ident = func(name.as_str());
                match field.sql_type {
                    SqlType::Text | SqlType::Varchar(_) => quote::quote! { Filter::#ident(v) => format!("{} {} '{}'", #name_orig, #op, v) },
                  SqlType::Integer
                                => quote::quote! { Filter::#ident(v) => format!("{} {} {}", #name_orig, #op, v) },
                }
             })
       }).collect::<Vec<TokenStream>>();
    let q_impl = quote::quote! {
        impl Filter {
            pub fn to_condition(filter: &Filter) -> String {
                match filter {
                    Filter::And(a, b) => format!("({} AND {})", Filter::to_condition(a), Filter::to_condition(b)),
                    Filter::Or(a, b)  => format!("({} OR  {})", Filter::to_condition(a), Filter::to_condition(b)),
                    #( #q_filter_matcher ),*
                }
            }
        }
    };

    let filter = impl_from_for_filter(input);

    quote::quote! {
      #q_enum
      #q_impl
      #filter
    }
}

fn impl_from_for_filter(input: &Input) -> proc_macro2::TokenStream {
    let ident = &input.ast.ident;

    let eq = |na: &str| syn::Ident::new(&format!("{}Equal", na), proc_macro2::Span::call_site());

    let q = input
        .fields_iter()
        .map(|field| {
            let ident = field.ident;
            let filter_ident = eq(format!("{}", ident)
                .to_case(convert_case::Case::Pascal)
                .as_str());
            quote::quote! {
              Filter::#filter_ident(item.#ident)
            }
        })
        .fold(None, |acc, q| match acc {
            None => Some(q),
            Some(o) => Some(quote::quote! { Filter::And( Box::new(#o), Box::new(#q) ) }),
        })
        .expect("Unable to construct filter");

    quote::quote! {
        impl From<#ident> for Filter {
            fn from(item: #ident) -> Filter {
                #q
            }
        }
    }
}
