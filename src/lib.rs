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
use syn;
use convert_case::{Case, Casing};

#[proc_macro_attribute]
pub fn impl_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    // println!("{:#}", attr);

    

    // let attr_ast = syn::parse(attr).expect("Attr failed");
    // println!("Hello");
    // let struct_ast = syn::parse(item).expect("Struct failed");
    // println!("Hello pt2");

    impl_gen_macro(&attr, &item);

    todo!()

}

fn impl_gen_macro(attrs: &TokenStream, structs: &TokenStream) -> TokenStream {
    let str_name = attrs.to_string();
    let names = str_name.split(',').collect::<Vec<_>>();
    // let name = &struct_ast.ident;
    for t in structs.into_iter() {
        println!("{}", t);
    };
    let fn_name = format_ident!("get_{}", names[1].trim().to_case(Case::Snake));
    let trait_name = format_ident!("{}", names[0].trim());
    let gen = quote! {
        impl #trait_name for Asd {
            fn get_id() -> &'static str {
                #fn_name
            }
        }
    };
    gen.into()
}
