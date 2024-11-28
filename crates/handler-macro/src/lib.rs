use {
    proc_macro::TokenStream,
    proc_macro2::Span,
    quote::{quote, ToTokens},
    std::env::var,
    syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Path, Token},
};

#[proc_macro]
pub fn handlers(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as Handlers)
        .to_token_stream()
        .into()
}

struct Handlers {
    instructions: Punctuated<Path, Token![,]>,
    is_pinocchio: bool,
}

impl Parse for Handlers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let instructions = Punctuated::<Path, Token![,]>::parse_terminated(input)?;

        Ok(Handlers {
            instructions,
            is_pinocchio: is_pinocchio()?,
        })
    }
}

impl ToTokens for Handlers {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let instructions = self.instructions.iter().enumerate().map(|(i, val)| {
            let i = i as u8;
            quote! {
                #i => crayfish_context::handle(accounts, instruction_data_inner, #val)?,
            }
        });

        let entrypoint = if self.is_pinocchio {
            quote! {
                pinocchio::entrypoint!(process_instruction);
            }
        } else {
            quote! {
                use solana_nostd_entrypoint::NoStdAccountInfo;

                solana_nostd_entrypoint::entrypoint_nostd!(process_instruction, 32);

                solana_nostd_entrypoint::noalloc_allocator!();
                solana_nostd_entrypoint::basic_panic_impl!();
            }
        };

        let expanded = quote! {
            #entrypoint

            pub fn process_instruction(
                _program_id: &crayfish_program::pubkey::Pubkey,
                accounts: &[crayfish_program::RawAccountInfo],
                instruction_data: &[u8],
            ) -> crayfish_program::ProgramResult {
                let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
                match instruction_discriminant[0] {
                    #(#instructions)*
                    _ => {
                        crayfish_program::msg!("Error: unknown instruction") //TODO
                    },
                }
                Ok(())
            }
        };

        expanded.to_tokens(tokens);
    }
}

fn is_pinocchio() -> syn::Result<bool> {
    let cargo_toml_path = get_cargo_toml()?;

    let content = std::fs::read_to_string(cargo_toml_path)
        .map_err(|_| syn::Error::new(Span::call_site(), "Cannot read the Cargo.toml file."))?;

    Ok(content.contains("features = [\"pinocchio\"]"))
}

fn get_cargo_toml() -> syn::Result<String> {
    let crate_dir = var("CARGO_MANIFEST_DIR")
        .map_err(|_| syn::Error::new(Span::call_site(), "Not in valid rust project."))?;

    Ok(format!("{crate_dir}/Cargo.toml"))
}
