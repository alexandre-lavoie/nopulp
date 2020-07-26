#[macro_use]
extern crate quote;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Ident, Token, ExprLit};
use syn::token::Brace;
use syn::braced;
use syn::Error;

struct HTMLProperty {
    key: Ident,
    value: Expr
}

struct HTMLElement {
    tag: Ident,
    value: Option<Expr>,
    properties: Vec<HTMLProperty>,
    children: Vec<HTMLElement>
}

impl Parse for HTMLElement {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let tag: Ident = input.parse()?;
        let mut properties: Vec<HTMLProperty> = Vec::new();
        let mut value: Option<Expr> = None;

        loop {
            if input.peek(Token![/]) || input.peek(Token![>]) {
                break;
            }

            let property: HTMLProperty = input.parse()?;

            properties.push(property);
        };

        let mut children: Vec<HTMLElement> = Vec::new();

        if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;

            if input.peek(Token![<]) {
                loop {
                    if input.peek(Token![<]) && input.peek2(Token![/]) {
                        break;
                    }
    
                    let child: HTMLElement = input.parse()?;
    
                    children.push(child);
                }
            } else {
                value = Some(if input.peek(Brace) {
                    let content;
                    braced!(content in input);
                    content.parse()?
                } else {
                    let lit: ExprLit = input.parse()?;
        
                    Expr::Lit(lit)
                });
            }

            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;

            let matching_tag: Ident = input.parse()?;

            if tag != matching_tag {
                return Err(Error::new_spanned(quote!{ #tag #matching_tag }, "Expected open tag to match close tag."));
            }

            input.parse::<Token![>]>()?;
        } else {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
        }

        Ok(
            HTMLElement {
                tag,
                value,
                properties,
                children
            }
        )
    }
}

impl Parse for HTMLProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;

        input.parse::<Token![=]>()?;

        let value: Expr = if input.peek(Brace) {
            let content;
            braced!(content in input);
            content.parse()?
        } else {
            let lit: ExprLit = input.parse()?;

            Expr::Lit(lit)
        };

        Ok(HTMLProperty {
            key, 
            value
        })
    }
}

impl HTMLProperty {
    fn to_tokens(&mut self) -> quote::__private::TokenStream {
        let HTMLProperty { key, value } = self;

        quote!(#key: #value)
    }
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl HTMLElement {
    fn to_tokens(&mut self) -> quote::__private::TokenStream {
        let HTMLElement { tag, value, properties, children } = self;

        let uppercase_string = uppercase_first_letter(tag.to_string().to_lowercase().as_str());

        let mapped_tag = Ident::new(uppercase_string.as_str(), tag.span());

        let mapped_value = if value.is_some() {
            let unwrap_value = value.as_ref();

            quote!(value: Some(#unwrap_value))
        } else {
            quote!(value: None)
        };

        let mapped_properties: Vec<quote::__private::TokenStream> = properties.into_iter().map(|x| x.to_tokens()).rev().collect();

        let mapped_children: Vec<quote::__private::TokenStream> = children.into_iter().map(|x| x.to_tokens()).rev().collect();

        quote!(Box::new(#mapped_tag {
            0: Object {
                style: Style {
                    #(#mapped_properties,)*
                    ..Style::default()
                },
                #mapped_value,
                children: vec![#(#mapped_children),*],
                ..Object::default()
            }
        }))
    }
}


#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let mut h: HTMLElement = parse_macro_input!(input as HTMLElement);

    let tokens = h.to_tokens();

    // eprintln!("{}", tokens.to_string()); // For logging output of HTML.

    tokens.into()
}