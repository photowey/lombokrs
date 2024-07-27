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

// setter

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use synext::{try_derive_input, try_parse_named_fields};

// ----------------------------------------------------------------

const SETTER_PREFIX: &str = "set_";

// ----------------------------------------------------------------

pub(crate) fn derive_setter(input: TokenStream) -> TokenStream {
    let derive_input = try_derive_input(input);
    let struct_name = &derive_input.ident;

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let mut setters = Vec::new();

    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        // set_x(..)
        // |- set_id(..)
        // |- set_name(..)
        // |- ...
        let setter_name = format_ident!("{}{}", SETTER_PREFIX, field_name);

        let setter = quote! {
            pub fn #setter_name(&mut self, #field_name: #field_type) {
                self.#field_name = #field_name;
            }
        };

        setters.push(setter);
    });

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(
                #setters
            )*
        }
    };

    let x = Some("");
    x.unwrap_or_default();

    TokenStream::from(expanded)
}
