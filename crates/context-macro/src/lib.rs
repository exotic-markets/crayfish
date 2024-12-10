use {
    accounts::{Account, Accounts},
    arguments::Arguments,
    lifetime::InjectLifetime,
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{
        parse::Parse, parse_macro_input, parse_quote, spanned::Spanned, visit_mut::VisitMut,
        Fields, Generics, Ident, Item, Lifetime,
    },
};

mod accounts;
mod arguments;
mod constraints;
mod lifetime;

#[proc_macro_attribute]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let context = parse_macro_input!(item as Context);

    TokenStream::from(context.into_token_stream())
}

// struct ContextAttributes {
//     args: Vec<String>,
// }

// impl Fold for ContextAttributes {
//     fn fold_item_struct(&mut self, _: syn::ItemStruct) -> syn::ItemStruct {
//         todo!()
//     }
// }

struct Context {
    ident: Ident,
    generics: Generics,
    item: Item,
    accounts: Accounts,
    args: Arguments,
}
impl Parse for Context {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: Item = input.parse()?;

        let mut injector = InjectLifetime;
        injector.visit_item_mut(&mut item);

        match item {
            Item::Struct(mut item_struct) => {
                let args = item_struct
                    .attrs
                    .iter_mut()
                    .filter(|attr| attr.meta.path().is_ident("args"))
                    .map(Arguments::try_from)
                    .collect::<Result<Vec<Arguments>, syn::Error>>()?
                    .first()
                    .unwrap_or(&Arguments::Values(vec![]))
                    .to_owned();

                let accounts = item_struct
                    .fields
                    .iter_mut()
                    .map(Account::try_from)
                    .collect::<Result<Vec<Account>, syn::Error>>()?;

                Ok(Context {
                    ident: item_struct.ident.to_owned(),
                    generics: item_struct.generics.to_owned(),
                    item: Item::Struct(item_struct),
                    accounts: Accounts(accounts),
                    args,
                })
            }
            Item::Enum(_item_enum) => todo!(), // TODO multiple context condition
            _ => Err(syn::Error::new(
                item.span(),
                "#[context] is only implemented for struct and enum",
            )),
        }
    }
}

impl ToTokens for Context {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let account_struct = &mut self.item.to_owned();
        let name = &self.ident;
        let generics = &self.generics;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let new_lifetime: Lifetime = parse_quote!('info);
        let (name_list, accounts_assign) = self.accounts.split_for_impl();
        let (args_struct_name, args_struct, args_assign) = self.args.split_for_impl(name);

        if let Item::Struct(account_struct) = account_struct {
            // Add an `args` field to the context
            if let Fields::Named(fields) = &mut account_struct.fields {
                fields.named.push(parse_quote! {
                    pub args: crayfish_context::args::Args<#new_lifetime, #args_struct_name>
                });
            }

            // Remove the args attribute
            account_struct
                .attrs
                .retain(|attr| !attr.meta.path().is_ident("args"));
        } else {
            return syn::Error::new(account_struct.span(), "Item is supposed to be a struct")
                .to_compile_error()
                .to_tokens(tokens);
        }

        let expanded = quote! {
            #args_struct

            #account_struct

            impl #impl_generics crayfish_context::HandlerContext<#new_lifetime> for #name #ty_generics #where_clause {
                fn from_entrypoint(
                    accounts: &mut &'info [crayfish_program::RawAccountInfo],
                    instruction_data: &mut &'info [u8],
                ) -> Result<Self, ProgramError> {
                    let [#name_list, rem @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys);
                    };

                    #args_assign
                    #accounts_assign

                    *accounts = rem;

                    Ok(#name { #name_list, args })
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}
