/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// getter

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::Type;

use crate::syntax::parser::{try_derive_input, try_named_fields};

// ----------------------------------------------------------------

const GETTER_PREFIX: &str = "get_";

// ----------------------------------------------------------------

pub(crate) fn derive_getter(input: TokenStream) -> TokenStream {
    let derive_input = try_derive_input(input);
    let struct_name = &derive_input.ident;

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    // Getters
    let mut getters = Vec::new();

    let fields = try_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        let return_type = match field_type {
            Type::Reference(ref_type) => ref_type.to_token_stream(),
            _ => quote! { &#field_type },
        };

        // get_x()
        // |- get_id()
        // |- get_name()
        // |- ...
        let getter_name = format_ident!("{}{}", GETTER_PREFIX, field_name);

        let getter = quote! {
            pub fn #getter_name(&self) -> #return_type {
                &self.#field_name
            }
        };
        getters.push(getter);

        // x()
        // |- id()
        // |- name()
        // |- ...
        let fluent_getter_name = format_ident!("{}", field_name);
        let fluent_getter = quote! {
            pub fn #fluent_getter_name(&self) -> #return_type {
                &self.#field_name
            }
        };
        getters.push(fluent_getter);
    });

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(
                #getters
            )*
        }
    };

    TokenStream::from(expanded)
}
