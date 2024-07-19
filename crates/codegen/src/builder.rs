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

// builder

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::syntax::parser::{try_derive_input, try_named_fields};

const BUILDER_SUFFIX: &str = "Builder";

pub fn derive_builder(input: TokenStream) -> TokenStream {
    let derive_input = try_derive_input(input);
    let struct_name = &derive_input.ident;

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let visibility = derive_input.vis.clone();

    let builder_name = format_ident!("{}{}", struct_name, BUILDER_SUFFIX);

    let mut builder_fields = Vec::new();
    let mut builder_setters = Vec::new();
    let mut build_fields = Vec::new();
    let mut builder_defaults = Vec::new();

    let fields = try_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        let builder_field = quote! {
            #field_name: Option<#field_type>
        };
        builder_fields.push(builder_field);

        let builder_setter = quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = Some(#field_name);
                self
            }
        };
        builder_setters.push(builder_setter);

        let build_field = quote! {
            #field_name: self.#field_name.unwrap_or_default()
        };
        build_fields.push(build_field);

        let builder_default = quote! {
            #field_name: ::core::default::Default::default()
        };
        builder_defaults.push(builder_default);
    });

    let expanded = quote! {
        #visibility struct #builder_name #ty_generics #where_clause {
            #(
                #builder_fields
            ),*
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn builder() -> #builder_name #ty_generics {
                #builder_name {
                    #(
                        #builder_defaults
                    ),*
                }
            }
        }

        impl #impl_generics #builder_name #ty_generics #where_clause {
            #(
                #builder_setters
            )*

            pub fn build(self) -> #struct_name #ty_generics {
                #struct_name {
                    #(
                        #build_fields
                    ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
