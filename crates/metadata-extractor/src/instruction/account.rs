use {
    crate::doc::Docs,
    syn::{visit::Visit, Field, GenericArgument, PathArguments, PathSegment, Type},
};

#[derive(Debug, Default)]
pub struct AccountFlags {
    is_signer: bool,
    is_mutable: bool,
    is_optional: bool,
}

#[derive(Debug)]
pub struct InstructionAccount {
    pub name: String,
    pub docs: Docs,
    pub flags: AccountFlags,
}

impl From<&Field> for InstructionAccount {
    fn from(value: &Field) -> Self {
        let mut flags = AccountFlags::default();
        extract_account_flags(&value.ty, &mut flags);

        let mut docs = Docs::default();
        value
            .attrs
            .iter()
            .for_each(|attr| docs.visit_attribute(attr));

        // TODO field with no name
        let name = value
            .ident
            .as_ref()
            .map(|i| i.to_string())
            .unwrap_or_default();

        InstructionAccount { name, docs, flags }
    }
}

fn extract_ty_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(ty_path) => ty_path.path.segments.last(),
        _ => None,
    }
}

fn extract_ty_from_arguments(args: &PathArguments) -> Option<&Type> {
    match args {
        PathArguments::AngleBracketed(generic_args) => {
            generic_args.args.first().and_then(|arg| match arg {
                GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
        }
        _ => None,
    }
}

fn extract_account_flags(ty: &Type, account_flags: &mut AccountFlags) {
    if let Some(PathSegment { ident, arguments }) = extract_ty_segment(ty) {
        match ident {
            i if i == "Option" => {
                if let Some(inner_ty) = extract_ty_from_arguments(arguments) {
                    account_flags.is_optional = true;
                    extract_account_flags(inner_ty, account_flags);
                }
            }
            i if i == "Mut" => {
                if let Some(inner_ty) = extract_ty_from_arguments(arguments) {
                    account_flags.is_mutable = true;
                    extract_account_flags(inner_ty, account_flags);
                }
            }
            i if i == "Signer" => account_flags.is_signer = true,
            _ => (),
        }
    }
}
