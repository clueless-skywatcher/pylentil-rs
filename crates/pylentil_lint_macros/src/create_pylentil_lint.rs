use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Token};
use syn::{Ident, LitStr, Result as SynResult, Token};

pub struct PyLintDetails {
    name: Ident,
    code: String,
    source: Ident,
    category: Ident,
    severity: Ident,
    message: String,
    possible_fix: Option<String>
}

impl Parse for PyLintDetails {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let code_litstr: LitStr = input.parse()?;
        let code = code_litstr.value();
        input.parse::<Token![,]>()?;
        let source: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let category: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let severity: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let message_litstr: LitStr = input.parse()?;
        let message = message_litstr.value();

        let possible_fix = if input.peek(Comma) {
            input.parse::<Token![,]>()?;
            let fix: LitStr = input.parse()?;
            Some(fix.value())
        } else {
            None
        };

        Ok(PyLintDetails {
            name,
            code,
            source,
            severity,
            category,
            message,
            possible_fix,
        })
    }
}

pub fn create_pylentil_lint_impl(input: PyLintDetails) -> TokenStream2 {
    let PyLintDetails {
        name,
        code,
        source,
        severity,
        category,
        message,
        possible_fix
    } = input;
    
    let source_object = match source.to_string().as_str() {
        "pylint" => quote! { LintSource::Pylint },
        "pycodestyle" => quote! { LintSource::Pycodestyle },
        _ => panic!("Unknown source"),
    };

    let category_object = match category.to_string().as_str() {
        "correctness" => quote! { LintCategory::Correctness },
        "style" => quote! { LintCategory::Style },
        "restriction" => quote! { LintCategory::Restriction },
        _ => panic!("Unknown category"),
    };

    let severity_object = match severity.to_string().as_str() {
        "warning" => quote! { LintSeverity::Warning },
        "error" => quote! { LintSeverity::Error },
        "nitpick" => quote! { LintSeverity::Nitpick },
        _ => panic!("Unknown severity")
    };

    let possible_fix_code = match possible_fix {
        Some(fix) => quote! { Some(#fix) },
        None => quote! { None },
    };

    quote! {
        use std::path::Path;

        use crate::{
            lint::{Lint, LintCategory, LintMeta, LintSeverity, LintSource},
            violation::LintViolation,
        };

        #[derive(Debug, Clone)]
        pub struct #name; 

        impl LintMeta for #name {
            const CODE: &'static str = #code;
            const SOURCE: LintSource = #source_object;
            const CATEGORY: LintCategory = #category_object;
            const SEVERITY: LintSeverity = #severity_object;
            const MESSAGE: &'static str = #message;
            const POSSIBLE_FIX: Option<&'static str> = #possible_fix_code;
        }
    }
}
