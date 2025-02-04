use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod grid;

#[proc_macro_derive(Grid, attributes(symbol))]
pub fn derive_grid(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    grid::expand_derive_grid(&input).into()
}
