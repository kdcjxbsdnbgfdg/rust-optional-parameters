#![allow(dead_code, unused_parens, unused_variables, non_snake_case)]

use quote::quote;
use syn::{FnArg, Pat, Token, parenthesized, parse::Parse, parse_macro_input};

struct DefaultParams(Vec<DefaultParam>);
struct DefaultParam {
    name: syn::Ident,
    value: syn::Expr,
}

struct FunctionParams(Vec<FunctionParam>);
struct FunctionParam(syn::Ident, syn::Type);
struct FunctionName(syn::Ident);
struct AnnotatedFunction(FunctionName, FunctionParams);

impl Parse for DefaultParam {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = syn::Ident::parse(input)?;
        _ = input.parse::<Token![=]>();
        let value = syn::Expr::parse(input)?;
        return Ok(DefaultParam { name, value });
    }
}

impl Parse for DefaultParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut params = Vec::new();
        let Ok(start) = DefaultParam::parse(input) else {
            return Ok(DefaultParams(params));
        };
        params.push(start);
        while input.parse::<Token![,]>().is_ok()
            && let Ok(param) = DefaultParam::parse(input)
        {
            params.push(param);
        }
        return Ok(DefaultParams(params));
    }
}

impl Parse for FunctionName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::Ident::parse(input).map(|n| FunctionName(n))
    }
}
impl Parse for AnnotatedFunction {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // function qualifiers
        if input.peek(Token![const]) {
            input.parse::<Token![const]>()?;
        }
        if input.peek(Token![async]) {
            input.parse::<Token![async]>()?;
        }
        if input.peek(Token![unsafe]) {
            input.parse::<Token![unsafe]>()?;
        }
        if input.peek(Token![extern]) {
            input.parse::<Token![extern]>()?;
            // ABI
            if input.peek(syn::LitStr) {
                input.parse::<syn::LitStr>()?;
            }
        }

        input.parse::<Token![fn]>()?;
        let name = FunctionName::parse(input)?;

        // parse generics
        input.parse::<syn::Generics>()?;

        let content;
        let paren = parenthesized!(content in input);
        let mut params = Vec::new();
        // TODO clean up
        if let Ok(start) = syn::FnArg::parse(&content) {
            match start {
                FnArg::Typed(meow) => {
                    let paramType = *meow.ty;
                    match *meow.pat {
                        Pat::Ident(ident) => {
                            let parameter = FunctionParam(ident.ident, paramType);
                            params.push(parameter);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        while content.parse::<Token![,]>().is_ok()
            && let Ok(arg) = syn::FnArg::parse(&content)
        {
            if let FnArg::Typed(arg) = arg
                && let Pat::Ident(ident) = *arg.pat
            {
                let paramType = *arg.ty;
                let parameter = FunctionParam(ident.ident, paramType);
                params.push(parameter);
            }
        }
        // syn requires that the entire input is parsed
        let _ = input.parse::<proc_macro2::TokenStream>()?;
        let params = FunctionParams(params);
        let annotatedFunction = AnnotatedFunction(name, params);
        return Ok(annotatedFunction);
    }
}

fn tokens(
    annotatedFunction: &AnnotatedFunction,
    defaultParams: &DefaultParams,
    stream: &mut proc_macro2::TokenStream,
) {
    let defaultValues = defaultParams.0.iter().map(|v| {
        let value = &v.value;
        quote! {
            #value
        }
    });
    let difference = annotatedFunction.1.0.len() - defaultParams.0.len();
    let paramsLen = annotatedFunction.1.0.len();
    let x = annotatedFunction
        .1
        .0
        .get(paramsLen - difference..paramsLen)
        .unwrap();
    let initialisations = x.iter().map(|x| {
        let paramType = &x.1;
        let paramName = &x.0;
        quote! {
            let mut #paramName: #paramType;
        }
    });
    let values = defaultParams.0.iter().map(|x| {
        let paramName = &x.name;
        let paramValue = &x.value;
        quote! {
            #paramName = #paramValue;
        }
    });
    let optionalParamNames = x.iter().map(|x| {
        let paramName = &x.0;
        quote! {
            #paramName
        }
    });
    // TODO how do i get rid of this clone
    let name = &annotatedFunction.0.0;
    stream.extend(quote! {
        macro_rules! #name {
            ($($arg:expr),* $(,)?) => {
                #name($($arg),* , #(#defaultValues),*)
            };
            ($($arg:expr),* , $(.$paramName:ident = $paramValue:expr),* ) => {
                (|| {
                    optional_params::unhygienic! {
                    #(#initialisations)*
                    #(#values)*
                    $($paramName = $paramValue;)*
                    #name($($arg),* , #(#optionalParamNames),*)
                    }
                })()
            };
        }
    });
}
// RUST SHUT THE FUCK UP!!! ITS NOT A TEST ITS FOR PEOPLE TO READ
// TO KNOW HOW IT WORKS

/// Allows you to create a function with optional parameters
/// annotate a function with 
/// ``` no_run
/// #[default_params(param = value, param = value)]
/// ```
/// to create a macro of the same name
/// the macro will take all of the required parameters (if any)
/// at the start, then you write 
/// ``` ignore
/// func!(.param = value);
/// ```
/// Note: the . is necessary
/// Note: rust will complain about unsued code, thats intentional (hard to fix)
#[proc_macro_attribute]
pub fn default_params(
    input: proc_macro::TokenStream,
    annotated_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // TODO theres probably a way to do this without cloning, like using fork()
    let annotated_function = annotated_item.clone();
    let annotated_function = parse_macro_input!(annotated_function as AnnotatedFunction);
    let defaultParams = parse_macro_input!(input as DefaultParams);
    let mut x: proc_macro2::TokenStream = annotated_item.into();
    tokens(&annotated_function, &defaultParams, &mut x);
    return x.into();
}

#[proc_macro]
pub fn unhygienic(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens.to_string().parse().unwrap()
}
