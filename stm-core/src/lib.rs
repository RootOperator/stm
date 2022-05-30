#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{Data, DeriveInput, Ident, Type};

#[proc_macro_derive(FromMap)]
pub fn from_map(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Implementation must be a struct"),
    };
    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let typecalls: Vec<Ident> = fields
        .iter()
        .map(|field| match field.ty.clone() {
            Type::Path(typepath) => {
                let typename: String = quote! {#typepath}.to_string().to_lowercase();
                Ident::new(&typename, Span::mixed_site())
            }
            _ => unimplemented!(),
        })
        .collect::<Vec<Ident>>();

    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {
        use stm::value::Value;
        use stm::{StringMap, GenericMap};

        impl #impl_generics FromMap for #name #ty_generics #where_clause {
            fn from_map(mut hashmap: GenericMap) -> #name {
                let mut settings = #name::default();
                #(
                    match hashmap.entry(String::from(#keys)) {
                        ::std::collections::hash_map::Entry::Occupied(entry) => {
                            let value = match entry.get().#typecalls() {
                                Some(val) => val,
                                None => panic!("Cannot parse out map entry")
                            };
                            settings.#idents = value;
                        },
                        _ => panic!("Cannot parse out map entry"),
                    }
                )*
                settings
            }
        }
    };
    TokenStream::from(tokens)
}

#[proc_macro_derive(ToMap)]
pub fn to_map(input_struct: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input_struct as DeriveInput);

    let fields = match ast.data {
        Data::Struct(st) => st.fields,
        _ => panic!("Implementation must be a struct"),
    };

    let idents: Vec<&Ident> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect::<Vec<&Ident>>();

    let keys: Vec<String> = idents
        .clone()
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let name: &Ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let tokens = quote! {

        impl #impl_generics ToMap for #name #ty_generics #where_clause {

            fn to_map(mut input_struct: #name) -> stm::StringMap {
                let mut map = stm::StringMap::new();
                #(
                    map.insert(#keys.to_string(), input_struct.#idents.to_string());
                )*
                map
            }
        }
    };
    TokenStream::from(tokens)
}
