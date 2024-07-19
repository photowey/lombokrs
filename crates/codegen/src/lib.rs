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

// lib

// ----------------------------------------------------------------

extern crate proc_macro;

use proc_macro::TokenStream;

use crate::builder::derive_builder;
use crate::getter::derive_getter;
use crate::setter::derive_setter;

// ----------------------------------------------------------------

mod builder;
mod getter;
mod setter;
mod syntax;

// ----------------------------------------------------------------

#[proc_macro_derive(Setter)]
pub fn setter_derive(input: TokenStream) -> TokenStream {
    derive_setter(input)
}

#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    derive_getter(input)
}

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    derive_builder(input)
}
