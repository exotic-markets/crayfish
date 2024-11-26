use {
    proc_macro2::Span,
    syn::{fold::Fold, parse_str, Lifetime, PathArguments},
};

pub struct InjectLifetime;

impl Fold for InjectLifetime {
    // fn fold_item(&mut self, i: syn::Item) -> syn::Item {
    //     match i {
    //         syn::Item::Enum(item_enum) => todo!(),
    //         syn::Item::Fn(item_fn) => todo!(),
    //         syn::Item::Impl(item_impl) => todo!(),
    //         // syn::Item::Struct(item_struct) => {}
    //         syn::Item::Trait(item_trait) => {
    //             // item_trait
    //             // item_trait.
    //             todo!()
    //         }
    //         _ => i,
    //     }
    // }

    // fn fold_generics(&mut self, i: syn::Generics) -> syn::Generics {
    //     for lifetime in i.lifetimes_mut() {
    //         lifetime.
    //     }
    // }

    fn fold_type_path(&mut self, mut i: syn::TypePath) -> syn::TypePath {
        if let Some(seg) = i.path.segments.last_mut() {
            match seg.arguments {
                PathArguments::AngleBracketed(ref mut gen_args) => {
                    gen_args.args.insert(
                        0,
                        syn::GenericArgument::Lifetime(Lifetime::new("'info", Span::call_site())),
                    );
                }
                PathArguments::None => {
                    if let Ok(lifetime) = parse_str("<'info>") {
                        seg.arguments = PathArguments::AngleBracketed(lifetime);
                    }
                }
                PathArguments::Parenthesized(_) => {}
            }
        }
        i
    }
}
