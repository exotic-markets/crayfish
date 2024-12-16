use {
    anchor_lang_idl_spec::Idl, five8_const::decode_32_const, proc_macro2::Span, quote::quote,
    syn::Ident,
};

fn gen_program_id(idl: &Idl) -> proc_macro2::TokenStream {
    let name = &idl.metadata.name;
    let ident = Ident::new(&format!("{name}Program"), Span::call_site());
    let program_id = &idl.address;
    let id_array = decode_32_const(program_id);

    // quote! {
    //     pub struct #ident;

    //     impl ProgramId for #ident {
    //         const ID: program::pubkey::Pubkey = Pubkey::from(#id_array);
    //     }
    // }
    quote! {}
}
