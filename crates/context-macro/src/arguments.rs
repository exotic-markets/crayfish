use {
    proc_macro2::TokenStream,
    quote::{format_ident, quote, ToTokens},
    syn::{
        parse::{Parse, ParseStream},
        parse2,
        spanned::Spanned,
        Attribute, Ident, Path, PathSegment, Token,
    },
};

#[derive(Clone, Debug)]
pub struct Argument {
    name: Ident,
    ty: PathSegment,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Path = input.parse()?;
        let path_segment = ty
            .segments
            .first()
            .ok_or_else(|| {
                syn::Error::new(ty.span(), "Expected at least one path segment for type")
            })?
            .clone();
        Ok(Argument {
            name,
            ty: path_segment,
        })
    }
}

pub struct Assign<'a>(Vec<(&'a Ident, &'a PathSegment)>);

impl ToTokens for Assign<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assign_fields = self.0.iter().map(|(name, ty)| {
            quote! {
                let #name = <#ty as crayfish_accounts::FromAccountInfo>::try_from_info(#name)?;
            }
        });

        let expanded = quote! {
            #(#assign_fields)*
        };

        expanded.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct Arguments(pub Vec<Argument>);

impl Arguments {
    pub fn generate_struct(&self, name: &Ident) -> (Ident, TokenStream, TokenStream) {
        let struct_name = format_ident!("{}Args", name);

        let fields = self.0.iter().map(|arg| {
            let name = &arg.name;
            let ty = &arg.ty.ident;
            quote! {
                pub #name: #ty,
            }
        });

        let generated_struct = quote! {
            #[repr(C)]
            #[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
            pub struct #struct_name {
                #(#fields)*
            }
        };

        let assign = quote! {
            let args = crayfish_context::args::Args::<#struct_name>::from_entrypoint(accounts, instruction_data)?;
        };

        (struct_name, generated_struct, assign)
    }
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arguments = Vec::new();
        while !input.is_empty() {
            let arg: Argument = input.parse()?;
            arguments.push(arg);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Arguments(arguments))
    }
}

impl TryFrom<&mut Attribute> for Arguments {
    type Error = syn::Error;

    fn try_from(value: &mut Attribute) -> Result<Self, Self::Error> {
        let tokens = value.meta.require_list()?.tokens.clone();
        Ok(parse2::<Arguments>(tokens)?)
    }
}
