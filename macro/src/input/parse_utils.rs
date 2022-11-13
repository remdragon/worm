use proc_macro_error::abort;
use syn::Lit;

pub fn parse_bool_lit(lit: &Lit) -> bool {
    match lit {
        Lit::Bool(b) => b.value,
        Lit::Str(str) => match str.value().as_str() {
            "true" => true,
            "false" => false,
            _ => {
                abort!(str, "Literal cannot be parsed as bool")
            }
        },
        Lit::Int(int) => match int.to_string().as_str() {
            "1" => true,
            "0" => false,
            _ => {
                abort!(int, "Literal cannot be parsed as bool")
            }
        },
        _ => {
            abort!(lit, "Literal cannot be parsed as bool")
        }
    }
}

pub fn parse_integer_lit(lit: &Lit) -> u64 {
    match lit {
        Lit::Str(str) => match str.value().as_str().parse::<u64>() {
            Ok(int) => int,
            _ => abort!(str, "Literal cannot be parsed as integer"),
        },
        Lit::Int(int) => match int.base10_parse() {
            Ok(integer) => integer,
            _ => {
                abort!(int, "Literal cannot be parsed as integer")
            }
        },
        _ => {
            abort!(lit, "Literal cannot be parsed as integer")
        }
    }
}
