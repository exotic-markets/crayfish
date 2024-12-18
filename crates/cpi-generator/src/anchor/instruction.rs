use {
    crate::anchor::{gen_docs, gen_type, gen_type_ref},
    anchor_lang_idl_spec::{Idl, IdlField, IdlInstructionAccountItem, IdlType},
    heck::ToUpperCamelCase,
    proc_macro2::{Span, TokenStream},
    quote::quote,
    syn::{Expr, Ident},
};

pub fn gen_instructions(idl: &Idl) -> TokenStream {
    let program_id = &idl.address;
    let instructions = idl.instructions.iter().map(|instruction| {
        let name = instruction.name.to_upper_camel_case();
        let ident = Ident::new(&name, Span::call_site());
        let (metas, accounts) = gen_account_instruction(&instruction.accounts);
        let docs = gen_docs(&instruction.docs);
        let program_result = gen_instructionn_result(&instruction.returns);

        let account_metas = gen_account_metas(&metas);
        let discriminator = &instruction.discriminator;
        let (arg_fields, instruction_data) =
            gen_instruction_data(&instruction.args, discriminator, program_id);

        quote! {
            /// Used for Cross-Program Invocation (CPI) calls.
            #docs
            pub struct #ident<'a> {
                #(pub #accounts: &'a program::RawAccountInfo,)*
                #(#arg_fields)*
            }

            impl #ident<'_> {
                #[inline(always)]
                pub fn invoke(&self) -> #program_result {
                    self.invoke_signed(&[])
                }

                pub fn invoke_signed(&self, signers: program::Signer) -> #program_result {
                    #account_metas
                    #instruction_data

                    program::invoke_signed(
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

fn gen_instruction_data(
    args: &[IdlField],
    discriminator: &[u8],
    program_id: &str,
) -> (Vec<TokenStream>, TokenStream) {
    let discriminator_len = discriminator.len();
    let buffer_size = 1232 - discriminator_len;
    let discriminator_expr: Expr = syn::parse_quote!(&[#(#discriminator),*]);
    let (arg_fields, arg_ser): (Vec<TokenStream>, Vec<TokenStream>) = args
        .iter()
        .map(|arg| {
            let ident = Ident::new(&arg.name, Span::call_site());
            let ty_ref = gen_type_ref(&arg.ty);

            (
                quote!(#ident: #ty_ref,),
                quote!(ident.serialize(&mut writer).map_err(|_| ProgramError::BorshIoError)?;),
            )
        })
        .unzip();

    let instruction_data = if arg_ser.is_empty() {
        quote! {
            let mut instruction_data = [program::UNINIT_BYTE; #buffer_size];
            let mut writer = MaybeUninitWriter::new(destination, #discriminator_len);

            write_bytes(&mut instruction_data, #discriminator_expr);
            #(#arg_ser)*

            let instruction = program::Instruction {
                program_id: &program::pubkey!(#program_id),
                accounts: &account_metas,
                data: writer.initialized(),
            };
        }
    } else {
        quote! {
            let mut instruction_data = [program::UNINIT_BYTE; #discriminator_len];

            write_bytes(&mut instruction_data, #discriminator_expr);

            let instruction = program::Instruction {
                program_id: &program::pubkey!(#program_id),
                accounts: &account_metas,
                data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, #discriminator_len) },
            };
        }
    };

    (arg_fields, instruction_data)
}

fn gen_instructionn_result(returns: &Option<IdlType>) -> TokenStream {
    match returns {
        Some(ty) => {
            let result_ty = gen_type(ty);
            quote!(Result<#result_ty, program::program_error::ProgramError>)
        }
        None => quote!(program::ProgramResult),
    }
}

fn gen_account_instruction(
    accounts: &[IdlInstructionAccountItem],
) -> (Vec<TokenStream>, Vec<syn::Ident>) {
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
                let ident = Ident::new(name, Span::call_site());
                let is_writable = account.writable;
                let is_signer = account.signer;

                metas.push(quote! {
                    program::ToMeta::to_meta(&self.#ident, #is_writable, #is_signer)
                });
                fields.push(ident);
            }
        }
    }

    (metas, fields)
}

#[inline]
fn gen_account_metas(metas: &[TokenStream]) -> TokenStream {
    let len = metas.len();

    quote! {
        let account_metas: [program::AccountMeta; #len] = [#(#metas),*];
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anchor_lang_idl_spec::{IdlInstructionAccount, IdlInstructionAccounts};

//     #[test]
//     fn test_gen_account_instruction_single() {
//         let accounts = vec![IdlInstructionAccountItem::Single(IdlInstructionAccount {
//             name: "test_account".to_string(),
//             writable: true,
//             signer: false,
//             docs: vec![],
//             optional: false,
//             address: None,
//             pda: None,
//             relations: vec![],
//         })];

//         let (metas, fields) = gen_account_instruction(&accounts);

//         assert_eq!(metas.len(), 1);
//         assert_eq!(fields.len(), 1);
//         assert_eq!(fields[0].to_string(), "test_account");
//     }

//     #[test]
//     fn test_gen_account_instruction_composite() {
//         let accounts = vec![IdlInstructionAccountItem::Composite(
//             IdlInstructionAccounts {
//                 name: "group".to_string(),
//                 accounts: vec![IdlInstructionAccountItem::Single(IdlInstructionAccount {
//                     name: "nested_account".to_string(),
//                     writable: true,
//                     signer: true,
//                     docs: vec![],
//                     optional: false,
//                     address: None,
//                     pda: None,
//                     relations: vec![],
//                 })],
//             },
//         )];

//         let (metas, fields) = gen_account_instruction(&accounts);

//         assert_eq!(metas.len(), 1);
//         assert_eq!(fields.len(), 1);
//         assert_eq!(fields[0].to_string(), "nested_account");
//     }

//     #[test]
//     fn test_gen_account_metas() {
//         let metas = vec![quote!(meta1), quote!(meta2)];

//         let result = gen_account_metas(&metas).to_string();
//         assert!(result.contains("let account_metas"));
//         assert!(result.contains("meta1"));
//         assert!(result.contains("meta2"));
//     }
// }
