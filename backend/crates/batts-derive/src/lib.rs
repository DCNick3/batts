use crate::emitter::Emitter;
use manyhow::manyhow;
use proc_macro2::TokenStream;

mod collect_ids;
mod emitter;

#[manyhow]
#[proc_macro_derive(CollectIds, attributes(collect_ids))]
pub fn derive_collect_ids(input: TokenStream) -> TokenStream {
    let mut emitter = Emitter::new();

    let Some(input) = emitter.handle(syn::parse2(input)) else {
        return emitter.finish_token_stream();
    };

    let result = collect_ids::derive_collect_ids_impl(&mut emitter, input);

    emitter.finish_token_stream_with(result)
}
