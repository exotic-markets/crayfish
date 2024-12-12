use {
    crate::anchor::{gen_docs, gen_type_ref},
    anchor_lang_idl_spec::{Idl, IdlInstructionAccountItem},
    heck::ToUpperCamelCase,
    proc_macro2::Span,
    quote::quote,
    syn::{Expr, Ident},
};

pub fn gen_instructions(idl: &Idl) -> proc_macro2::TokenStream {
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

            impl #ident<'_> {
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
                let ident = Ident::new(name, Span::call_site());
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
