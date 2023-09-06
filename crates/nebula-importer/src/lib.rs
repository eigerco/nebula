use proc_macro::TokenStream;
use quote::quote;
use syn::ItemMod;

#[proc_macro_attribute]
pub fn import(attr: TokenStream, item: TokenStream) -> TokenStream {
    let module: ItemMod = syn::parse(item).expect("Could not read token");
    let file = if attr.to_string().len() == 0 {
        format!(".soroban/{}.wasm", module.ident.to_string())
    } else {
        format!(".soroban/{}.wasm", attr.to_string())
    };
    let name = module.ident;
    let tokens = quote! {
        mod #name {
            soroban_sdk::contractimport!(
                file = #file
            );
        }
    };
    tokens.into()
}
