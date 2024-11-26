use accounts::{Account, Accounts};
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Generics, Ident, Item, Lifetime};

mod accounts;
mod constraints;
mod lifetime;

// pub struct ContextAttributes {
//     args: Vec<String>,
// }

// impl Fold for ContextAttributes {
//     fn fold_item_struct(&mut self, i: syn::ItemStruct) -> syn::ItemStruct {
//         // //
//         // i.
//         // todo!()
//     }
// }

pub struct Context {
    ident: Ident,
    generics: Generics,
    item: Item,
    accounts: Accounts,
}
impl Parse for Context {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item: Item = input.parse()?;

        match item {
            Item::Struct(mut item_struct) => {
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
        let base_item = &self.item;
        let name = &self.ident;
        let generics = &self.generics;

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let new_lifetime = Lifetime::new("'a", Span::call_site());
        let (name_list, assign) = self.accounts.split_for_impl();

        let expanded = quote! {
            #base_item

            impl #impl_generics crayfish_context::HandlerContext<#new_lifetime> for #name #ty_generics #where_clause {
                fn from_entrypoint(
                    accounts: &mut &'a [crayfish_program::RawAccountInfo],
                    instruction_data: &mut &'a [u8],
                ) -> Result<Self, ProgramError> {
                    let [#name_list, rem @ ..] = accounts else {
                        return Err(ProgramError::NotEnoughAccountKeys);
                    };

                    #assign

                    *accounts = rem;

                    Ok(#name { #name_list })
                }
            }
        };
        expanded.to_tokens(tokens);
    }
}
