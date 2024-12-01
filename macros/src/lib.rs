use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprArray, Ident, LitStr};

#[proc_macro]
pub fn call_task(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as MacroInput);
    
    let MacroInput {
        year_prefix,
        day_prefix,
        year_range,
        day_range,
        function_name,
    } = input;

    for year in year_range.elems.iter() {
        
    }

    let output = quote! {
        fn call_function_based_on_input() {
            match (some_input_1, some_input_2) {
                #(#match_cases)*
                _ => {
                    println!("No matching function found!");
                }
            }
        }
    };

    output.into()
}

struct MacroInput {
    year_prefix: LitStr,
    day_prefix: LitStr,
    year_range: ExprArray,
    day_range: ExprArray,
    function_name: Ident,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year_prefix = input.parse()?;
        let day_prefix = input.parse()?;
        let year_range = input.parse::<Expr>()?;
        let day_range = input.parse::<Expr>()?;
        let function_name = input.parse()?;

        if let Expr::Array(year_range) = year_range {
            if let Expr::Array(day_range) = day_range {
               Ok(MacroInput {
                    year_prefix,
                    day_prefix,
                    year_range,
                    day_range,
                    function_name,
                })
            }
            else {
                Err(syn::Error::new_spanned(day_range, "Expected an array expression"))
            }
        } else {
            Err(syn::Error::new_spanned(year_range, "Expected an array expression"))
        }

    }
}
