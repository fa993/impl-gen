#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{self, Item};
use convert_case::{Case, Casing};

#[proc_macro_attribute]
pub fn impl_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    let struct_ast = syn::parse::<Item>(item).expect("Could not parse item");

    impl_gen_macro(&attr, &struct_ast)
}

fn impl_gen_macro(attrs: &TokenStream, struct_ast: &syn::Item) -> TokenStream {
    let name_token = match &struct_ast {
        Item::Type(t) => &t.ident,
        Item::Struct(t) => &t.ident, 
        _ => {println!("Nothing"); todo!()}

    };
    let str_name = attrs.to_string();
    let names = str_name.split(',').collect::<Vec<_>>();
    let fn_name = format_ident!("{}", names[1].trim().to_case(Case::Snake));
    let trait_name = format_ident!("{}", names[0].trim());
    let gen = quote! {

        #struct_ast

        impl #trait_name for #name_token {
            fn get_id() -> &'static str {
                #fn_name
            }
        }
    };
    gen.into()
}
