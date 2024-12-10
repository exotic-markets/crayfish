use {
    anchor_lang_idl_spec::{
        Idl, IdlAccount, IdlArrayLen, IdlDefinedFields, IdlField, IdlMetadata, IdlRepr,
        IdlReprModifier, IdlSerialization, IdlType, IdlTypeDef, IdlTypeDefTy, IDL_SPEC,
    },
    crayfish_metadata_extractor::{
        account::AccountState,
        definition::{DefinedField, DefinedFields, TypeDef},
        program::Program,
        ty::{FloatSize, Len, NumberSize, Type},
    },
    std::path::Path,
};

pub fn generate<P: AsRef<Path>>(file: P) {
    // let file = File::open(path)
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<Idl> for Program {
    fn convert(self) -> Idl {
        let (idl_accounts, idl_types) = self.accounts.into_iter().map(|a| a.convert()).unzip();

        Idl {
            address: self.program_id,
            metadata: IdlMetadata {
                name: "contract".to_string(),
                version: "0.1.0".to_string(),
                spec: IDL_SPEC.to_string(),
                description: None,
                repository: None,
                dependencies: vec![],
                contact: None,
                deployments: None,
            },
            docs: vec![],
            instructions: vec![],
            accounts: idl_accounts,
            types: idl_types,
            constants: vec![],
            errors: vec![],
            events: vec![],
        }
    }
}

impl Convert<(IdlAccount, IdlTypeDef)> for AccountState {
    fn convert(self) -> (IdlAccount, IdlTypeDef) {
        (
            IdlAccount {
                name: self.name.clone(),
                discriminator: vec![], //TODO discriminator
            },
            IdlTypeDef {
                name: self.name,
                docs: self.docs.into_vec(),
                generics: vec![], //TODO generics
                repr: Some(IdlRepr::C(IdlReprModifier {
                    packed: false,
                    align: Some(8),
                })),
                serialization: IdlSerialization::Bytemuck,
                ty: self.ty_def.convert(),
            },
        )
    }
}

impl Convert<IdlTypeDefTy> for TypeDef {
    fn convert(self) -> IdlTypeDefTy {
        match self {
            TypeDef::Struct { fields } => IdlTypeDefTy::Struct {
                fields: fields.convert(),
            },
            TypeDef::Enum { variants } => todo!(),
            TypeDef::Type { alias } => todo!(),
        }
    }
}

impl Convert<Option<IdlDefinedFields>> for DefinedFields {
    fn convert(self) -> Option<IdlDefinedFields> {
        match self {
            DefinedFields::Named(fields) => Some(IdlDefinedFields::Named(
                fields.into_iter().map(Convert::convert).collect(),
            )),
            DefinedFields::Tuple(types) => Some(IdlDefinedFields::Tuple(
                types.into_iter().map(Convert::convert).collect(),
            )),
            DefinedFields::Unit => None,
        }
    }
}

impl Convert<IdlField> for DefinedField {
    fn convert(self) -> IdlField {
        IdlField {
            name: self.name,
            docs: vec![],
            ty: self.ty.convert(),
        }
    }
}

impl Convert<IdlType> for Type {
    fn convert(self) -> IdlType {
        match self {
            Type::Bool => IdlType::Bool,
            Type::Number { signed, size } => match size {
                NumberSize::One => {
                    if signed {
                        IdlType::I8
                    } else {
                        IdlType::U8
                    }
                }
                NumberSize::Two => {
                    if signed {
                        IdlType::I16
                    } else {
                        IdlType::U16
                    }
                }
                NumberSize::Four => {
                    if signed {
                        IdlType::I32
                    } else {
                        IdlType::U32
                    }
                }
                NumberSize::Eight => {
                    if signed {
                        IdlType::I64
                    } else {
                        IdlType::U64
                    }
                }
                NumberSize::Sixteen => {
                    if signed {
                        IdlType::I128
                    } else {
                        IdlType::U128
                    }
                }
            },
            Type::Float { size } => match size {
                FloatSize::Four => IdlType::F32,
                FloatSize::Eight => IdlType::F64,
            },
            Type::Pubkey => IdlType::Pubkey,
            Type::String => IdlType::String,
            Type::Vec(ty) => {
                if matches!(
                    *ty,
                    Type::Number {
                        signed: false,
                        size: NumberSize::One
                    }
                ) {
                    IdlType::Bytes
                } else {
                    IdlType::Vec(Box::new(ty.convert()))
                }
            }
            Type::Array(ty, len) => {
                let array_len = match len {
                    Len::Number(size) => IdlArrayLen::Value(size),
                    Len::Const(name) => IdlArrayLen::Generic(name),
                };
                IdlType::Array(Box::new(ty.convert()), array_len)
            }
            Type::Option(ty) => IdlType::Option(Box::new(ty.convert())),
            Type::Defined(name) => IdlType::Defined {
                name,
                generics: vec![],
            }, //TODO generics
        }
    }
}
