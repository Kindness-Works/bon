use crate::util;
use crate::util::prelude::*;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Token;

pub(crate) fn generate(entries: Punctuated<Expr, Token![,]>) -> TokenStream2 {
    let error = util::validate_expressions_are_unique("value in the set", &entries);
    let entries = entries.into_iter();
    let output = quote! {
        ::core::iter::FromIterator::from_iter([
            #(::core::convert::Into::into(#entries)),*
        ])
    };

    // We unconditionally return `output` as part of the result to make sure IDEs
    // see this output and see what input tokens map to what output tokens. This
    // way IDEs can provide better help to the developer even when there are errors.
    error
        .map(|err| {
            let err = err.write_errors();
            quote! {{
                #err
                #output
            }}
        })
        .unwrap_or(output)
}
