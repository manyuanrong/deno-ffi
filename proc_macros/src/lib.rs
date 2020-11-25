extern crate proc_macro;
use proc_macro::{Span, TokenStream, TokenTree};
use std::iter;
use syn;

static API_CALL_TEMPLATE: &str = r#"
let api: fn($1) -> RP = unsafe { lib.symbol(name) }.map_err(|err| err.to_string())?;
Ok(api($2))
"#;

fn error_stream(msg: &str) -> TokenStream {
    TokenStream::from(syn::Error::new(Span::call_site().into(), msg).to_compile_error())
}

/// Expected usage:
/// api_call(5)
#[proc_macro]
pub fn api_call(item: TokenStream) -> TokenStream {
    let tokens = item.into_iter().collect::<Vec<TokenTree>>();
    if tokens.len() != 1 {
        return error_stream("expect single argument");
    }
    let tt = tokens.get(0).unwrap();
    let count = match tt {
        TokenTree::Literal(lit) => {
            let maybe_val = lit.to_string().parse::<usize>();
            if maybe_val.is_err() {
                return error_stream("expect a number");
            }
            maybe_val.unwrap()
        }
        _ => {
            return error_stream("unexpected input");
        }
    };

    let api_params_str = iter::repeat("RP").take(count).collect::<Vec<_>>().join(", ");
    let mut api_argument_list: Vec<String> = Vec::new();
    for i in 0..count {
        api_argument_list.push(format!("get_param(params, {})", i));
    }
    let api_arguments_str = api_argument_list.join(", ");
    let code = API_CALL_TEMPLATE.replace("$1", &api_params_str).replace("$2", &api_arguments_str);
    code.parse().unwrap()
}