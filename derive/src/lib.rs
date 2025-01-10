mod reflect;

use proc_macro::TokenStream;
use reflect::impl_reflect_macro;

#[proc_macro_derive(Reflect)]
pub fn reflect_macro(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();

    impl_reflect_macro(ast)
}
