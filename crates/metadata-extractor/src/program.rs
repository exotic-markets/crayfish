use crate::{account::AccountState, parsing::ParsingContext};

#[derive(Debug)]
pub struct Program {
    pub program_id: String,
    // pub instructions:
    pub accounts: Vec<AccountState>,
}

impl<'a> TryFrom<ParsingContext<'a>> for Program {
    type Error = syn::Error;

    fn try_from(value: ParsingContext<'a>) -> Result<Self, Self::Error> {
        let mut accounts = Vec::new();
        for item in value.file.items.iter() {
            if let syn::Item::Struct(item_struct) = item {
                if value.accounts.contains(&&item_struct.ident) {
                    accounts.push(AccountState::try_from(item_struct)?);
                }
            }
        }

        Ok(Program {
            program_id: "defualt".to_string(),
            accounts,
        })
    }
}
