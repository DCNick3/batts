use crate::emitter::Emitter;
use manyhow::emit;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Token;
use syn::{Attribute, DeriveInput};

struct CollectIdsStructAttr {
    types: Vec<syn::Type>,
}

impl syn::parse::Parse for CollectIdsStructAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types = input
            .parse_terminated(|input| input.parse(), Token![,])?
            .into_iter()
            .collect();

        Ok(Self { types })
    }
}

impl CollectIdsStructAttr {
    pub fn from_attributes(emitter: &mut Emitter, attributes: &[Attribute]) -> Self {
        let mut types = Vec::new();

        for attr in attributes {
            if attr.path().is_ident("collect_ids") {
                if let Some(args) = emitter.handle(attr.parse_args::<CollectIdsStructAttr>()) {
                    types.extend(args.types)
                };
            }
        }

        if types.is_empty() {
            emit!(
                emitter,
                "No types specified for CollectIds. Use `#[collect_ids(Type1, Type2)]`"
            )
        }

        Self { types }
    }
}

enum CollectIdsFieldAttrToken {
    Skip,
}

struct SpannedCollectIdsFieldAttrToken {
    span: Span,
    token: CollectIdsFieldAttrToken,
}

impl syn::parse::Parse for SpannedCollectIdsFieldAttrToken {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.step(|cursor| {
            let Some((ident, new_cursor)) = cursor.ident() else {
                return Err(cursor.error("expected identifier"));
            };
            match ident.to_string().as_str() {
                "skip" => Ok((
                    SpannedCollectIdsFieldAttrToken {
                        span: ident.span(),
                        token: CollectIdsFieldAttrToken::Skip,
                    },
                    new_cursor,
                )),
                _ => Err(cursor.error("expected `skip`")),
            }
        })
    }
}

#[derive(Default)]
struct CollectIdsFieldAttr {
    skip: bool,
}

impl CollectIdsFieldAttr {
    pub fn from_attributes(emitter: &mut Emitter, attributes: &[Attribute]) -> Self {
        let mut skip = false;

        for attr in attributes {
            if attr.path().is_ident("collect_ids") {
                let Some(list) = emitter.handle(attr.meta.require_list()) else {
                    continue;
                };
                let Some(tokens) = emitter.handle(list.parse_args_with(
                    Punctuated::<SpannedCollectIdsFieldAttrToken, Token![,]>::parse_terminated,
                )) else {
                    continue;
                };

                for token in tokens {
                    match token.token {
                        CollectIdsFieldAttrToken::Skip => {
                            if skip {
                                emit!(
                                    emitter,
                                    token.span,
                                    "Duplicate `skip` attribute on field. \
                                        Use `#[collect_ids(skip)]` only once per field."
                                )
                            }
                            skip = true;
                        }
                    }
                }
            }
        }

        Self { skip }
    }
}

pub fn derive_collect_ids_impl(emitter: &mut Emitter, input: DeriveInput) -> TokenStream {
    let CollectIdsStructAttr { types } =
        CollectIdsStructAttr::from_attributes(emitter, &input.attrs);

    let s = synstructure::Structure::new(&input);

    let mut result = TokenStream::new();
    for ty in types {
        let body = s.each_variant(|var| {
            let var_attr = if let syn::Data::Enum(_) = &input.data {
                CollectIdsFieldAttr::from_attributes(emitter, var.ast().attrs)
            } else {
                CollectIdsFieldAttr::default()
            };
            if var_attr.skip {
                return quote!();
            }

            let mut body = TokenStream::new();
            for bi in var.bindings() {
                syn::token::Brace::default().surround(&mut body, |body| {
                    let field_attr = CollectIdsFieldAttr::from_attributes(emitter, &bi.ast().attrs);
                    if field_attr.skip {
                        return;
                    }
                    quote!(
                        crate::related_data::CollectIds::<#ty>::collect_ids(#bi, ids);
                    )
                    .to_tokens(body);
                });
            }

            body
        });

        result.extend(s.gen_impl(quote! {
            gen impl crate::related_data::CollectIds<#ty> for @Self {
                fn collect_ids(&self, ids: &mut ::indexmap::IndexSet<#ty>) {
                    match self {
                        #body
                    }
                }
            }
        }))
    }

    result
}
