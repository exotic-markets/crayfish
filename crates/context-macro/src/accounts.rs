use {
    crate::constraints::Constraints,
    proc_macro2::{Span, TokenStream},
    quote::{quote, ToTokens},
    syn::{spanned::Spanned, visit_mut::VisitMut, Field, Ident, PathSegment, Type, TypePath},
};

pub struct Account {
    name: Ident,
    constraints: Constraints,
    ty: PathSegment,
}

impl TryFrom<&mut Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &mut Field) -> Result<Self, Self::Error> {
        let mut constraints = Constraints::default();
        constraints.visit_attributes_mut(&mut value.attrs);

        let segment = match &value.ty {
            Type::Path(TypePath { path, .. }) => path.segments.last(),
            _ => None,
        }
        .ok_or_else(|| syn::Error::new(value.span(), "Invalid type for the account"))?;

        let name = value
            .ident
            .clone()
            .unwrap_or(Ident::new("random", Span::call_site())); //TODO unit type

        Ok(Account {
            name,
            constraints,
            ty: segment.clone(),
        })
    }
}

pub struct NameList<'a>(Vec<&'a Ident>);

impl ToTokens for NameList<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names = &self.0;
        let expanded = quote! {
            #(#names),*
        };

        expanded.to_tokens(tokens);
    }
}

pub struct Assign<'a>(Vec<(&'a Ident, &'a PathSegment, &'a Constraints)>);

impl ToTokens for Assign<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assign_fields = self.0.iter().map(|(name, ty, c)| {
            if c.has_init() {
                let payer = c.get_payer();
                let space = c.get_space();

                let (Some(payer), Some(space)) = (payer, space) else {
                    return syn::Error::new(name.span(), "Not found payer or space for the init constraint").to_compile_error()
                };

                if let (Some(punctuated_seeds), Some(bump)) = (c.get_seeds(), c.get_bump()) {
                    quote! {
                        let #name: #ty = {
                            let system_acc = <crayfish_accounts::Mut<crayfish_accounts::SystemAccount> as crayfish_accounts::FromAccountInfo>::try_from_info(#name)?;
                            let signer_seeds = [#punctuated_seeds, &[#bump as u8]];
                            let seeds_vec = &signer_seeds.into_iter().map(|seed| crayfish_program::instruction::Seed::from(seed)).collect::<Vec<crayfish_program::instruction::Seed>>()[..];
                            let signer: crayfish_program::instruction::Signer = crayfish_program::instruction::Signer::from(&seeds_vec[..]);
                            crayfish_traits::SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, Some(&[crayfish_program::instruction::Signer::from(signer)]))?;
                            Mut::try_from_info(#name)?
                        };
                    }
                } else {
                    quote! {
                        let #name: #ty = {
                            let system_acc = <crayfish_accounts::Mut<crayfish_accounts::SystemAccount> as crayfish_accounts::FromAccountInfo>::try_from_info(#name)?;
                            crayfish_traits::SystemCpi::create_account(&system_acc, &#payer, &crate::ID, #space as u64, None)?;
                            Mut::try_from_info(#name)?
                        };
                    }
                }
            } else {
                quote! {
                    let #name = <#ty as crayfish_accounts::FromAccountInfo>::try_from_info(#name)?;
                }
            }
        });

        let expanded = quote! {
            #(#assign_fields)*
        };

        expanded.to_tokens(tokens);
    }
}

pub struct Verify<'a>(Vec<(&'a Ident, &'a Constraints)>);

impl ToTokens for Verify<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let verify_fields = self.0.iter().map(|(name, c)| {
            if (c.get_seeds().is_some() && c.get_bump().is_none()) || (c.get_seeds().is_some() && c.get_bump().is_none()) {
                return syn::Error::new(name.span(), "`seeds` and `bump` must be used together").to_compile_error();
            };

            if let (Some(punctuated_seeds), Some(bump)) = (c.get_seeds(), c.get_bump()) {
                quote! {
                    let pk = crayfish_program::pubkey::create_program_address(&[#punctuated_seeds, &[#bump as u8]], &crate::ID)?;
                    if #name.key() != &pk {
                        return Err(ProgramError::InvalidSeeds);
                    }
                }
            } else {
                quote!()
            }
        });

        let expanded = quote! {
            #(#verify_fields)*
        };
        expanded.to_tokens(tokens);
    }
}

pub struct Accounts(pub Vec<Account>);

impl Accounts {
    pub fn split_for_impl(&self) -> (NameList, Assign, Verify) {
        let iter = self.0.iter();

        (
            NameList(iter.clone().map(|el| &el.name).collect()),
            Assign(
                iter.clone()
                    .map(|el| (&el.name, &el.ty, &el.constraints))
                    .collect(),
            ),
            Verify(iter.map(|el| (&el.name, &el.constraints)).collect()),
        )
    }
}
