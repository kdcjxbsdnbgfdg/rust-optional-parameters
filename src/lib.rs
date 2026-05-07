//#![allow(dead_code, unused_parens, unused_variables, non_snake_case)]
//#![allow(unused_parens, non_snake_case, unused_variables, unused)]
#![allow(non_snake_case)]

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
        let _= parenthesized!(content in input);
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

    //let requiredParamsLen = annotatedFunction.1.0.len() - defaultParams.0.len();
    let optionalParamsLen = defaultParams.0.len();
    let paramsLen = annotatedFunction.1.0.len();
    // change the name to something better
    let optionalParameters = annotatedFunction
        .1
        .0
        .get(paramsLen - optionalParamsLen..paramsLen)
        .unwrap();

    let initialisations = optionalParameters.iter().map(|x| {
        let paramType = &x.1;
        let paramName = &x.0;
        let paramName = &quote::format_ident!("_{}", paramName);
        quote! {
            let mut #paramName: #paramType;
        }
    });
    let values = defaultParams.0.iter().map(|x| {
        let paramName = &x.name;
        let paramName = &quote::format_ident!("_{}", paramName);
        let paramValue = &x.value;
        quote! {
            #paramName = #paramValue;
        }
    });
    let optionalParamNames = optionalParameters.iter().map(|x| {
        let paramName = &x.0;
        let paramName = &quote::format_ident!("_{}", paramName);
        quote! {
            #paramName
        }
    });
    // i think i could just do something like implementing to tokens instead of this
    let initialisations2 = initialisations.clone();
    let values2= values.clone();
    let optionalParamNames2 = optionalParamNames.clone();
    let initialisations3 = initialisations.clone();
    let values3= values.clone();
    let optionalParamNames3 = optionalParamNames.clone();
    // TODO how do i get rid of this clone
    let name = &annotatedFunction.0.0;
    stream.extend(quote! {
        macro_rules! #name {
            () => {
                (|| {
                    // does this even need to be unhygeinic??
                    optional_params::unhygienic! {
                    #(#initialisations3)*
                    #(#values3)*
                    #name( #(#optionalParamNames3),*)
                    }
                })()
            };
            ( $(.$paramName:ident = $paramValue:expr),* ) => {
                (|| {
                    optional_params::unhygienic! {
                    paste::paste! {
                    #(#initialisations2)*
                    #(#values2)*
                    //$(_$paramName = $paramValue;)*
                    //$( underscore!{$paramName} = $paramValue;)*
                    $( [<_ $paramName>] = $paramValue;)*
                    #name( #(#optionalParamNames2),*)
                    }
                    }
                })()
            };
            ($($arg:expr),* $(,)?) => {
                #name($($arg),* , #(#defaultValues),*)
            };
            ($($arg:expr),* , $(.$paramName:ident = $paramValue:expr),* ) => {
                (|| {
                    optional_params::unhygienic! {
                    paste::paste! {
                    #(#initialisations)*
                    #(#values)*
                    //$(_$paramName = $paramValue;)*
                    $( [<_ $paramName>] = $paramValue;)*
                    #name($($arg),* , #(#optionalParamNames),*)
                    }
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

// maybe i can use set_span to change the hygiene of my proc macro variables?
#[proc_macro]
pub fn unhygienic(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tokens.to_string().parse().unwrap()
}

//#[proc_macro]
//pub fn underscore(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
//    let x: proc_macro::TokenStream = (String::from("_") + &tokens.to_string()).parse().unwrap();
//    return x;
//    //println!("{}", x.)
//}
