use syn::{
    parse::{Parse, ParseStream},
    visit_mut::VisitMut,
    Expr, Ident, Token,
};

//TODO rewrite it to add custom constraint for users
pub enum Constraint {
    Init(ConstraintInit),
    Payer(ConstraintPayer),
    Space(ConstraintSpace),
}

#[derive(Default)]
pub struct Constraints(pub Vec<Constraint>);

impl VisitMut for Constraints {
    fn visit_attributes_mut(&mut self, attrs: &mut Vec<syn::Attribute>) {
        self.0.reserve(attrs.len());

        attrs.retain(|attr| {
            if !attr.path().is_ident("constraint") {
                return true;
            }

            if let Ok(mut constraints) = attr.parse_args_with(parse_constraints) {
                self.0.append(&mut constraints);
            }

            false
        });
    }
}

impl Constraints {
    pub fn has_init(&self) -> bool {
        self.0
            .iter()
            .find(|c| matches!(c, Constraint::Init(_)))
            .is_some()
    }

    pub fn get_payer(&self) -> Option<&Expr> {
        self.0.iter().find_map(|c| {
            if let Constraint::Payer(ConstraintPayer { target }) = c {
                Some(target)
            } else {
                None
            }
        })
    }

    pub fn get_space(&self) -> Option<&Expr> {
        self.0.iter().find_map(|c| {
            if let Constraint::Space(ConstraintSpace { space }) = c {
                Some(space)
            } else {
                None
            }
        })
    }
}

pub fn parse_constraints(input: ParseStream) -> syn::Result<Vec<Constraint>> {
    let mut constraints = Vec::new();

    while !input.is_empty() {
        let name: Ident = input.parse()?;
        match name {
            i if i == "init" => {
                constraints.push(Constraint::Init(ConstraintInit));
            }
            i if i == "payer" => {
                constraints.push(Constraint::Payer(ConstraintPayer::parse(input)?));
            }
            i if i == "space" => {
                constraints.push(Constraint::Space(ConstraintSpace::parse(input)?));
            }
            _ => return Err(syn::Error::new(input.span(), "Unknow constraint.")),
        }

        if input.peek(Token![,]) {
            let _punct: Token![,] = input.parse()?;
        }
    }

    Ok(constraints)
}

pub struct ConstraintPayer {
    pub target: Expr,
}

impl Parse for ConstraintPayer {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let target = input.parse()?;

        Ok(ConstraintPayer { target })
    }
}

pub struct ConstraintSpace {
    pub space: Expr,
}

impl Parse for ConstraintSpace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _punct: Token![=] = input.parse()?;
        let space = input.parse()?;

        Ok(ConstraintSpace { space })
    }
}

pub struct ConstraintInit;
