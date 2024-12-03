pub mod anchor;

use {
    anchor::gen_docs,
    anchor_lang_idl_spec::{
        Idl, IdlAccount, IdlInstructionAccount, IdlInstructionAccountItem, IdlType, IdlTypeDef,
        IdlTypeDefTy,
    },
    heck::ToUpperCamelCase,
    proc_macro2::Span,
    quote::quote,
    syn::{parse_quote, Ident, Type},
};

pub fn generate_cpi(idl: &Idl) {
    // idl.

    // idl.types

    let expanded = quote! {
        pub mod cpi {

        }
    };
    // idl.instructions
    //     .iter()
    //     .map(|ins| ins.args.iter().map(|i| i.ty))
}

//TODO pubkey
// fn gen_program_id(idl: &Idl) -> proc_macro2::TokenStream {
//     let name = &idl.metadata.name;
//     let ident = Ident::new(&format!("{name}Program"), Span::call_site());
//     let program_id = &idl.address;

//     quote! {
//         pub struct #ident;

//         impl crayfish_accounts::ProgramId for #ident {
//             const ID: crayfish_program::pubkey::Pubkey = crate::ID;
//         }
//     }
// }

fn gen_type(idl_ty: IdlType) -> Type {
    match idl_ty {
        IdlType::Bool => parse_quote!(bool),
        IdlType::U8 => parse_quote!(u8),
        IdlType::I8 => parse_quote!(i8),
        IdlType::U16 => parse_quote!(u16),
        IdlType::I16 => parse_quote!(i16),
        IdlType::U32 => parse_quote!(u32),
        IdlType::I32 => parse_quote!(i32),
        IdlType::F32 => parse_quote!(f32),
        IdlType::U64 => parse_quote!(u64),
        IdlType::I64 => parse_quote!(i64),
        IdlType::F64 => parse_quote!(f64),
        IdlType::U128 => parse_quote!(u128),
        IdlType::I128 => parse_quote!(i128),
        IdlType::U256 => unimplemented!(),
        IdlType::I256 => unimplemented!(),
        IdlType::Bytes => parse_quote!(&[u8]),
        IdlType::String => parse_quote!(&str),
        IdlType::Pubkey => parse_quote!(&Pubkey),
        IdlType::Option(idl_type) => todo!(),
        IdlType::Vec(idl_type) => todo!(),
        IdlType::Array(idl_type, _) => {
            let ty = gen_type(*idl_type);
            parse_quote!(&[#ty])
        }
        IdlType::Defined { name, generics } => todo!(),
        IdlType::Generic(_) => todo!(),
        _ => todo!(),
    }
}

fn gen_account_instruction(
    accounts: &[IdlInstructionAccountItem],
) -> Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    accounts
        .iter()
        .map(|account| {
            match account {
                IdlInstructionAccountItem::Composite(composite_accounts) => {
                    let nested_accounts = gen_account_instruction(&composite_accounts.accounts);
                    let (metas, fields): (Vec<_>, Vec<_>) = nested_accounts.into_iter().unzip();

                    (quote! { #(#metas)* }, quote! { #(#fields)* })
                }
                IdlInstructionAccountItem::Single(account) => {
                    // Generate metadata and field for a single account
                    let name = &account.name;
                    let docs = gen_docs(&account.docs);
                    let is_writable = account.writable;
                    let is_signer = account.signer;

                    (
                        quote! {
                            crayfish_program::ToMeta(self.#name, #is_writable, #is_signer),
                        },
                        quote! {
                            #docs
                            pub #name: &'a crayfish_program::RawAccountInfo,
                        },
                    )
                }
            }
        })
        .collect()
}

fn gen_instructions(idl: &Idl) -> proc_macro2::TokenStream {
    let instructions = idl.instructions.iter().map(|i| {
        let name = i.name.to_upper_camel_case();
        let (metas, accounts): (Vec<_>, Vec<_>) =
            gen_account_instruction(&i.accounts).into_iter().unzip();
        let discriminator = &i.discriminator;
        let docs = gen_docs(&i.docs);

        let program_result = if let Some(_result) = &i.returns {
            quote!(Result<(), crayfish_program::program_error::ProgramError>)
        } else {
            quote!(crayfish_program::ProgramResult)
        };

        quote! {
            #docs
            pub struct #name<'a> {
                #(#accounts)*

            }

            impl #name {
                #[inline(always)]
                pub fn invoke(&self) -> #program_result {
                    self.invoke_signed(&[])
                }

                pub fn invoke_signed(&self, signers: &[&[&[u8]]]) -> #program_result {

                }
            }
        }
    });
    quote! {}
}

fn gen_owner(IdlAccount { name, .. }: IdlAccount) -> proc_macro2::TokenStream {
    // idl_account.discriminator TODO
    quote! {
        impl crayfish_accounts::Owner for #name {
            const OWNER: crayfish_program::pubkey::Pubkey = crate::ID;
        }
    }
}

// fn gen_type(idl_ty: IdlTypeDef) -> proc_macro2::TokenStream {
//     let docs = gen_docs(&idl_ty.docs);
//     // idl_ty.
//     match idl_ty.ty {
//         IdlTypeDefTy::Struct { fields } => todo!(),
//         IdlTypeDefTy::Enum { variants } => todo!(),
//         IdlTypeDefTy::Type { alias } => todo!(),
//     }

//     quote! {
//         #docs
//     }
// }
