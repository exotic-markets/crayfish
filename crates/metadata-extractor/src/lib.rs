pub mod account;
pub mod definition;
pub mod doc;
pub mod instruction;
pub mod parsing;
pub mod program;
pub mod ty;

pub use {doc::*, instruction::*};

// pub struct StateField<'a> {
//     pub name: &'a Ident,
//     pub ty: Type,
// }

// pub struct State<'a> {
//     pub name: &'a Ident,
//     pub fields: Vec<String>,
// }

// #[derive(Debug)]
// pub enum ContextField {
//     Account(InstructionAccount),
//     Args,
//     Bumps,
// }

// #[derive(Debug)]
// pub struct Context<'a> {
//     pub name: &'a Ident,
//     pub fields: Vec<ContextField>,
//     pub args: Option<&'a Ident>,
// }

// pub struct Intruction<'a> {
//     pub name: &'a Ident,
//     // pub context: Vec<Context>,
// }

// pub struct Program {
//     //TODO
// }

// impl Program {
//     fn from_file(file: &File) -> Program {
//         let context = ParsingContext::from(file);
//         let (accounts, contexts) = Self::extract_accounts_and_contexts(file, &context);
//         Program {}
//     }

//     fn extract_accounts_and_contexts<'a>(
//         file: &'a File,
//         context: &ParsingContext<'a>,
//     ) -> (Vec<Account<'a>>, Vec<Context<'a>>) {
//         let mut accounts = Vec::with_capacity(context.accounts.len());
//         let mut contexts = Vec::with_capacity(context.contexts.len());

//         for item in &file.items {
//             if let Item::Struct(item_struct) = item {
//                 let name = &item_struct.ident;

//                 if context.accounts.contains(&&item_struct.ident) {
//                     // item_struct.fields.iter().map(|f| {
//                     //     match f.ty {

//                     //     }
//                     // });
//                     accounts.push(Self::parse_account(name, &item_struct.attrs));
//                 }

//                 if context.contexts.contains(&&item_struct.ident) {
//                     contexts.push(Self::parse_context(name, &item_struct.fields));
//                 }
//             }
//         }

//         (accounts, contexts)
//     }

//     fn parse_account<'a>(name: &'a Ident, attrs: &[syn::Attribute]) -> Account<'a> {
//         let mut docs = Docs::default();
//         attrs.iter().for_each(|attr| docs.visit_attribute(attr));

//         Account { name, docs }
//     }

//     fn parse_context<'a>(name: &'a Ident, fields: &'a syn::Fields) -> Context<'a> {
//         let fields = fields
//             .iter()
//             .map(|f| {
//                 let mut docs = Docs::default();
//                 f.attrs.iter().for_each(|attr| docs.visit_attribute(attr));

//                 let account = Account {
//                     name: f.ident.as_ref().unwrap(),
//                     docs,
//                 };

//                 ContextField::Account(account)
//             })
//             .collect();

//         Context {
//             name,
//             fields,
//             args: None,
//         }
//     }
// }
