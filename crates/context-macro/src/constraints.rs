use syn::{parse::Parse, visit::Visit, Ident, Token};

#[derive(Default)]
pub struct Constraints(Vec<String>);

impl<'ast> Visit<'ast> for Constraints {
    fn visit_attribute(&mut self, i: &'ast syn::Attribute) {
        if !i.path().is_ident("account") {
            return;
        }

        // panic!("{i:?}");

        // i.parse_args()

        // match name.to_string().as_str() {
        //     "init" => (),
        //     "pda" => (),
        //     _ => (),
        // }
        // i.
    }
}

pub enum ConstraintList {
    Init,
    ATA,
}

impl Parse for ConstraintList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        match name {
            i if i == "init" => {
                InitConstraint::parse(input)?;
            }
            i if i == "seed" => {
                input.parse::<Token![=]>()?;
            }
            _ => {}
        }

        Ok(ConstraintList::Init)
    }
}

pub struct InitConstraint {
    // pub payer:
}

impl Parse for InitConstraint {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
