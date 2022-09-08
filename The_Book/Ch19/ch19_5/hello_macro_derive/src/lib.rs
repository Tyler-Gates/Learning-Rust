use proc_macro::TokenStream;
use quote::quote;
use syn;

//procedural macros need to be in their own crate. That is what this is!
//the convention is if you have a crate named foo, a custom derive procedural macro crate would be called foo_derive
//what you see in this crate is practically boilerplate for every procedural macro crate, parsing token stream, then a call to a function that transforms the syntax tree
//when any type has [derive(HelloMacro)] on a type this function will be called, indicated by syntax: #[proc_macro_derive(HelloMacro)]
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    //quote! macro lets us define the rust code we want to return
    let gen = quote! {
        //#name will be replaced with the value of name
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    //changes the quote to type tokenstream
    gen.into()
}