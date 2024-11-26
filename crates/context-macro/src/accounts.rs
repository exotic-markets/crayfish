use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, visit::Visit, Field, Ident, PathSegment, Type, TypePath};

use crate::constraints::{ConstraintList, Constraints};

pub struct Account {
    name: Ident,
    constraints: Vec<String>,
    ty: PathSegment,
}

impl TryFrom<&Field> for Account {
    type Error = syn::Error;

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        let mut constraints = Constraints::default();
        for attr in &value.attrs {
            //Add constraintes here
            constraints.visit_attribute(attr);
        }

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
            constraints: vec![],
            ty: segment.clone(),
        })
    }
}

pub struct NameList<'a>(Vec<&'a Ident>);

impl<'a> ToTokens for NameList<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let names = &self.0;
        let expanded = quote! {
            #(#names),*
        };

        expanded.to_tokens(tokens);
    }
}

pub struct Assign<'a>(Vec<(&'a Ident, &'a PathSegment)>);

impl<'a> ToTokens for Assign<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assign_fields = self.0.iter().map(|(name, ty)| {
            quote! {
                let #name: #ty = <#ty as crayfish_accounts::FromAccountInfo>::try_from_info(#name)?;
            }
        });

        let expanded = quote! {
            #(#assign_fields)*
        };

        expanded.to_tokens(tokens);
    }
}

pub struct Accounts(pub Vec<Account>);

impl Accounts {
    pub fn split_for_impl(&self) -> (NameList, Assign) {
        let (name_list, assign): (Vec<&Ident>, Vec<(&Ident, &PathSegment)>) = self
            .0
            .iter()
            .map(|el| (&el.name, (&el.name, &el.ty)))
            .unzip();

        (NameList(name_list), Assign(assign))
    }
}
