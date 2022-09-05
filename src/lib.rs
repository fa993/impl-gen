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

    impl_gen_macro(&attr, &struct_ast);

    todo!()

}

fn impl_gen_macro(attrs: &TokenStream, struct_ast: &syn::Item) -> TokenStream {
    let name_token = match struct_ast {
        Item::Type(t) => &t.ident,
        Item::Struct(t) => &t.ident, 
        Item::Const(_) => {println!("Const"); todo!()},
        Item::Enum(_) => {println!("Enum"); todo!()},
        Item::ExternCrate(_) => {println!("ExternCrate"); todo!()},
        Item::Fn(_) => {println!("Fn"); todo!()},
        Item::ForeignMod(_) => {println!("ForeignMod"); todo!()},
        Item::Impl(_) => {println!("Impl"); todo!()},
        Item::Macro(_) => {println!("Macro"); todo!()},
        Item::Macro2(_) => {println!("Macro2"); todo!()},
        Item::Mod(_) => {println!("Mod"); todo!()},
        Item::Static(_) => {println!("Static"); todo!()},
        Item::Trait(_) => {println!("Trait"); todo!()},
        Item::TraitAlias(_) => {println!("TraitAlias"); todo!()},
        Item::Union(_) => {println!("Union"); todo!()},
        Item::Use(_) => {println!("Use"); todo!()},
        Item::Verbatim(_) => {println!("Verbatim"); todo!()},
        _ => {println!("Nothing"); todo!()}

    };
    let str_name = attrs.to_string();
    let names = str_name.split(',').collect::<Vec<_>>();
    let fn_name = format_ident!("get_{}", names[1].trim().to_case(Case::Snake));
    let trait_name = format_ident!("{}", names[0].trim());
    let gen = quote! {
        impl #trait_name for #name_token {
            fn get_id() -> &'static str {
                #fn_name
            }
        }
    };
    gen.into()
}
