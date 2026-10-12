mod create_pylentil_lint;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::create_pylentil_lint::PyLintDetails;


#[proc_macro]
pub fn create_pylentil_lint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PyLintDetails);
    create_pylentil_lint::create_pylentil_lint_impl(input).into()
}