#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn impl_gen(attr: TokenStream, item: TokenStream) -> TokenStream {
    
    println!("{:#}", attr);

    let attr_ast = syn::parse(attr).expect("Attr failed");
    let struct_ast = syn::parse(item).expect("Struct failed");

    impl_gen_macro(&attr_ast, &struct_ast);

    todo!()

}

fn impl_gen_macro(attr_ast: &syn::DeriveInput, struct_ast: &syn::DeriveInput) -> TokenStream {
    let name = &struct_ast.ident;
    // println!("{:?}", attr_ast);
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
