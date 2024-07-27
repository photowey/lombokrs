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
use syn::spanned::Spanned;
use syn::{DeriveInput, Type};
use synext::{
    try_derive_input, try_parse_named_fields, try_predicate_is_option, try_predicate_is_vec,
    try_unwrap_option, try_unwrap_vec,
};

// ----------------------------------------------------------------

const BUILDER_SUFFIX: &str = "Builder";
const BUILDER_ATTR_NAME: &str = "builder";
const BUILDER_ATTR_CUSTOM_METHOD: &str = "method";

// ----------------------------------------------------------------

pub fn derive_builder_fields(derive_input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut builder_fields = Vec::new();
    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        if try_predicate_is_option(field_type) {
            let inner_type = try_unwrap_option(field_type);
            let builder_field = quote! {
                #field_name: ::std::option::Option<#inner_type>
            };

            builder_fields.push(builder_field);
        } else if try_predicate_is_vec(field_type) {
            let builder_field = quote! {
                #field_name: #field_type
            };

            builder_fields.push(builder_field);
        } else {
            let builder_field = quote! {
                #field_name: ::std::option::Option<#field_type>
            };

            builder_fields.push(builder_field);
        }
    });

    builder_fields
}

pub fn derive_builder_setters(derive_input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut builder_setters = Vec::new();
    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        if try_predicate_is_option(field_type) {
            let inner_type = try_unwrap_option(field_type);
            let builder_setter = quote! {
                pub fn #field_name(mut self, #field_name: #inner_type) -> Self {
                    self.#field_name = ::std::option::Option::Some(#field_name);
                    self
                }
            };

            builder_setters.push(builder_setter);
        } else if try_predicate_is_vec(field_type) {
            let builder_setter = quote! {
                pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name.extend(#field_name);
                    self
                }
            };
            builder_setters.push(builder_setter);

            // #[builder(method = "activity")]
            // activities: Vec<String>
            if let Ok(builder_method_opt) = try_predicate_vec_builder_custom_method(field) {
                if let Some(builder_method) = builder_method_opt {
                    let inner_type = try_unwrap_vec(field_type);

                    let builder_method_setter = quote! {
                        pub fn #builder_method(mut self, #builder_method: #inner_type) -> Self {
                            self.#field_name.push(#builder_method);
                            self
                        }
                    };
                    builder_setters.push(builder_method_setter);
                }
            }
        } else {
            let builder_setter = quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = ::std::option::Option::Some(#field_name);
                    self
                }
            };

            builder_setters.push(builder_setter);
        }
    });

    builder_setters
}

pub fn derive_builder_defaults(derive_input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut builder_defaults = Vec::new();
    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        if try_predicate_is_vec(field_type) {
            let builder_default = quote! {
                #field_name: ::std::vec::Vec::new()
            };
            builder_defaults.push(builder_default);
        } else {
            let builder_default = quote! {
                #field_name: ::std::option::Option::None
            };
            builder_defaults.push(builder_default);
        }
    });

    builder_defaults
}

pub fn derive_build_field_checker(derive_input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut build_field_checkers = Vec::new();
    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        if try_predicate_is_not_option_and_vec(field_type) {
            let field_checker = quote! {
                if self.#field_name.is_none() {
                    let err = format!("Missing field: `{}`!", stringify!(#field_name));
                    return ::std::result::Result::Err(err.into())
                }
            };
            build_field_checkers.push(field_checker);
        }
    });

    build_field_checkers
}

pub fn derive_build_fields(derive_input: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut build_fields = Vec::new();
    let fields = try_parse_named_fields(&derive_input);
    fields.iter().for_each(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;

        if try_predicate_is_option(field_type) {
            let build_field = quote! {
                #field_name: self.#field_name.clone().unwrap()
            };
            build_fields.push(build_field);
        } else if try_predicate_is_vec(field_type) {
            let build_field = quote! {
                #field_name: self.#field_name.clone()
            };
            build_fields.push(build_field);
        } else {
            let build_field = quote! {
                #field_name: self.#field_name.clone().unwrap()
            };
            build_fields.push(build_field);
        }
    });

    build_fields
}

pub fn derive_builder(input: TokenStream) -> TokenStream {
    let derive_input = try_derive_input(input);
    let struct_name = &derive_input.ident;

    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let visibility = derive_input.vis.clone();

    let builder_name = format_ident!("{}{}", struct_name, BUILDER_SUFFIX);

    let builder_fields = derive_builder_fields(&derive_input);
    let builder_setters = derive_builder_setters(&derive_input);
    let builder_defaults = derive_builder_defaults(&derive_input);
    let build_field_checkers = derive_build_field_checker(&derive_input);
    let build_fields = derive_build_fields(&derive_input);

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

            pub fn build(self) -> ::std::result::Result<#struct_name #ty_generics, ::std::boxed::Box<dyn std::error::Error + 'static>> {
                #(
                    #build_field_checkers
                )*

                ::std::result::Result::Ok(
                    #struct_name {
                        #(
                            #build_fields
                        ),*
                    }
                )
            }
        }
    };

    TokenStream::from(expanded)
}

// ----------------------------------------------------------------

fn try_predicate_is_not_option_and_vec(ty: &Type) -> bool {
    try_predicate_is_not_option(ty) && try_predicate_is_not_vec(ty)
}

fn try_predicate_is_not_option(ty: &Type) -> bool {
    !try_predicate_is_option(ty)
}

fn try_predicate_is_not_vec(ty: &Type) -> bool {
    !try_predicate_is_vec(ty)
}

fn try_predicate_vec_builder_custom_method(field: &syn::Field) -> syn::Result<Option<syn::Ident>> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == BUILDER_ATTR_NAME {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident(BUILDER_ATTR_CUSTOM_METHOD) {
                            if let syn::Lit::Str(ref method) = kv.lit {
                                return Ok(Some(syn::Ident::new(
                                    method.value().as_str(),
                                    attr.span(),
                                )));
                            }
                        } else {
                            if let Ok(syn::Meta::List(ref list)) = attr.parse_meta() {
                                return Err(syn::Error::new_spanned(
                                    list,
                                    format!(
                                        r#"expected `{}({} = "...")`"#,
                                        BUILDER_ATTR_NAME, BUILDER_ATTR_CUSTOM_METHOD
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}
