pub mod anchor;

use {
    anchor::gen_docs,
    anchor_lang_idl_spec::{Idl, IdlAccount, IdlGenericArg, IdlInstructionAccountItem, IdlType},
    heck::ToUpperCamelCase,
    proc_macro2::Span,
    quote::quote,
    syn::{parse_quote, Expr, Ident, Type},
};

pub fn generate_cpi(idl: &Idl) -> proc_macro2::TokenStream {
    let instructions = gen_instructions(idl);

    quote! {
        #instructions
    }
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

fn gen_type_ref(idl_ty: &IdlType) -> Type {
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
        IdlType::Bytes => parse_quote!(&'a [u8]),
        IdlType::String => parse_quote!(&'a str),
        IdlType::Pubkey => parse_quote!(&'a Pubkey),
        IdlType::Option(inner) => {
            let ty = gen_type_ref(inner);
            parse_quote!(Option<#ty>)
        }
        IdlType::Vec(inner) | IdlType::Array(inner, _) => {
            let ty = gen_type_ref(inner);
            parse_quote!(&'a [#ty])
        }
        IdlType::Defined { name, generics } => {
            if generics.is_empty() {
                parse_quote!(&'a #name)
            } else {
                let generic_types = generics.iter().map(|g| match g {
                    IdlGenericArg::Type { ty } => gen_type_ref(ty),
                    IdlGenericArg::Const { value } => parse_quote!(#value),
                });
                parse_quote!(&'a #name<#(#generic_types),*>)
            }
        }
        IdlType::U256 | IdlType::I256 | IdlType::Generic(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

fn gen_account_instruction(
    accounts: &[IdlInstructionAccountItem],
) -> (Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let mut metas = Vec::with_capacity(accounts.len());
    let mut fields = Vec::with_capacity(accounts.len());

    for account in accounts {
        match account {
            IdlInstructionAccountItem::Composite(composite_accounts) => {
                let (nested_metas, nested_fields) =
                    gen_account_instruction(&composite_accounts.accounts);
                metas.extend(nested_metas);
                fields.extend(nested_fields);
            }
            IdlInstructionAccountItem::Single(account) => {
                let name = &account.name;
                let ident = Ident::new(&name, Span::call_site());
                let is_writable = account.writable;
                let is_signer = account.signer;

                metas.push(quote! {
                    crayfish_program::ToMeta::to_meta(&self.#ident, #is_writable, #is_signer)
                });
                fields.push(ident);
            }
        }
    }

    (metas, fields)
}

fn gen_instructions(idl: &Idl) -> proc_macro2::TokenStream {
    let program_id = &idl.address;
    let instructions = idl.instructions.iter().map(|instruction| {
        let name = instruction.name.to_upper_camel_case();
        let ident = Ident::new(&name, Span::call_site());
        let (metas, accounts) = gen_account_instruction(&instruction.accounts);
        let docs = gen_docs(&instruction.docs);

        let program_result = match &instruction.returns {
            Some(ty) => {
                let result_ty = gen_type_ref(ty);
                quote!(Result<#result_ty, crayfish_program::program_error::ProgramError>)
            }
            None => quote!(crayfish_program::ProgramResult),
        };

        let account_metas = gen_account_metas(&metas);
        let discriminator = &instruction.discriminator;
        let discriminator_len = discriminator.len();
        let discriminator_expr: Expr = syn::parse_quote!(&[#(#discriminator),*]);
        let data_len = discriminator_len;

        quote! {
            /// Used for Cross-Program Invocation (CPI) calls.
            #docs
            pub struct #ident<'a> {
                #(pub #accounts: &'a crayfish_program::RawAccountInfo,)*
            }

            impl<'a> #ident<'a> {
                #[inline(always)]
                pub fn invoke(&self) -> #program_result {
                    self.invoke_signed(&[])
                }

                pub fn invoke_signed(&self, signers: crayfish_program::Signer) -> #program_result {
                    #account_metas

                    let mut instruction_data = [crayfish_program::UNINIT_BYTE; #data_len];

                    write_bytes(&mut instruction_data[..#discriminator_len], #discriminator_expr);

                    let instruction = crayfish_program::Instruction {
                        program_id: &crayfish_program::pubkey!(#program_id),
                        accounts: &account_metas,
                        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, #data_len) },
                    };

                    crayfish_program::invoke_signed(
                        &instruction,
                        &[#(self.#accounts),*],
                        signers
                    )
                }
            }
        }
    });

    quote! {
        #(#instructions)*
    }
}

// fn gen_instruction_args() -> proc_macro2::TokenStream {}

#[inline]
fn gen_account_metas(metas: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream {
    if metas.is_empty() {
        quote! {
            let account_metas: [crayfish_program::AccountMeta; 0] = [];
        }
    } else {
        let metas_len = metas.len();
        quote! {
            let account_metas: [crayfish_program::AccountMeta; #metas_len] = [
                #(#metas),*
            ];
        }
    }
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
