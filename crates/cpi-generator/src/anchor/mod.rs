use quote::quote;

pub fn gen_docs(docs: &[String]) -> proc_macro2::TokenStream {
    let docs = docs
        .iter()
        .map(|doc| format!("{}{doc}", if doc.is_empty() { "" } else { " " }))
        .map(|doc| quote! { #[doc = #doc] });
    quote! { #(#docs)* }
}
