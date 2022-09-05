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
use syn::{self, Item, Ident};
use convert_case::{Case, Casing};

#[proc_macro_attribute]
pub fn impl_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    let struct_ast = syn::parse::<Item>(item).expect("Could not parse item");

    let name_token = match struct_ast {
        Item::Type(t) => {
            t.ident
        },
        Item::Struct(t) => {
            t.ident
        } 
        _ => {
            unimplemented!();
        }
    };

    impl_gen_macro(&attr, &name_token);

    todo!()

}

fn impl_gen_macro(attrs: &TokenStream, name: &Ident) -> TokenStream {
    let str_name = attrs.to_string();
    let names = str_name.split(',').collect::<Vec<_>>();
    let fn_name = format_ident!("get_{}", names[1].trim().to_case(Case::Snake));
    let trait_name = format_ident!("{}", names[0].trim());
    let gen = quote! {
        impl #trait_name for #name {
            fn get_id() -> &'static str {
                #fn_name
            }
        }
    };
    gen.into()
}
