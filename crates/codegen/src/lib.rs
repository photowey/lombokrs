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

// ----------------------------------------------------------------

/// `Setter` is a macro that helps you automatically generate setter methods for structs.
///
/// - set_x()
///   - set_id()
///   - ...
///
/// # Examples
///
/// ```rust
/// use lombokrs_codegen::{Getter, Setter};
/// // use lombokrs::*;
///
/// #[derive(Setter, Getter, Debug)]
/// pub struct User {
///     id: u32,
///     age: u8,
///     name: String,
///     email: String,
///     hobby: Vec<String>,
/// }
///
/// // with lifetime
/// #[derive(Setter, Getter, Debug)]
/// pub struct LifetimeUser<'a> {
///     id: u32,
///     age: u8,
///     name: &'a str,
///     email: &'a str,
///     hobby: Box<&'a str>,
/// }
///
/// // ----------------------------------------------------------------
///
/// impl User {
///     pub fn new(id: u32, age: u8, name: String, email: String, hobby: Vec<String>) -> Self {
///         Self {
///             id,
///             age,
///             name,
///             email,
///             hobby,
///         }
///     }
/// }
///
/// // ----------------------------------------------------------------
///
/// // Usage:
///
///     let mut user = User::new(
///         10086,
///         18,
///         "photowey".to_string(),
///         "photowey@gmail.com".to_string(),
///         vec!["badminton".to_string()],
///     );
///
///     user.set_id(9527);
///     user.set_age(25);
///     user.set_name("lombokrs".to_string());
///     user.set_email("lombokrs@gmail.com".to_string());
///     user.set_hobby(vec!["football".to_string()]);
///
///     assert_eq!(&9527u32, user.get_id());
///     assert_eq!(&25u8, user.get_age());
///
///     assert_eq!("lombokrs", user.get_name());
///     assert_eq!("lombokrs@gmail.com", user.get_email());
///     assert_eq!(&vec!["football".to_string()], user.get_hobby());
///
/// ```
#[proc_macro_derive(Setter)]
pub fn setter_derive(input: TokenStream) -> TokenStream {
    derive_setter(input)
}

/// `Getter` is a macro that helps you automatically generate getter methods for structs.
///
/// - get_x()
///   - get_id()
///   - ...
/// - x()
///   - id()
///   - ...
///
/// # Examples
///
/// ```rust
/// use lombokrs_codegen::{Getter, Setter};
/// // use lombokrs::*;
///
/// #[derive(Setter, Getter, Debug)]
/// pub struct User {
///     id: u32,
///     age: u8,
///     name: String,
///     email: String,
///     hobby: Vec<String>,
/// }
///
/// // with lifetime
/// #[derive(Setter, Getter, Debug)]
/// pub struct LifetimeUser<'a> {
///     id: u32,
///     age: u8,
///     name: &'a str,
///     email: &'a str,
///     hobby: Box<&'a str>,
/// }
///
/// // ----------------------------------------------------------------
///
/// impl User {
///     pub fn new(id: u32, age: u8, name: String, email: String, hobby: Vec<String>) -> Self {
///         Self {
///             id,
///             age,
///             name,
///             email,
///             hobby,
///         }
///     }
/// }
///
/// // ----------------------------------------------------------------
///
/// // Usage:
///
///    let user = User::new(
///         10086,
///         18,
///         "photowey".to_string(),
///         "photowey@gmail.com".to_string(),
///         vec!["badminton".to_string()],
///     );
///
///     assert_eq!(&10086u32, user.get_id());
///     assert_eq!(&18u8, user.get_age());
///
///     assert_eq!("photowey", user.get_name());
///     assert_eq!("photowey@gmail.com", user.get_email());
///     assert_eq!(&vec!["badminton".to_string()], user.get_hobby());
/// ```
#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    derive_getter(input)
}

/// `Getter` is a macro that helps you automatically generate `XxxBuilder` struct and chain methods for structs.
///
/// # Examples
///
/// ```rust
/// use lombokrs_codegen::{Builder, Getter, Setter};
/// // use lombokrs::*;
///
/// #[derive(Setter, Getter, Builder, Debug)]
/// pub struct User {
///     id: u32,
///     age: u8,
///     name: String,
///     email: String,
///     hobby: Vec<String>,
///     // @since 0.2.0
///     #[builder(method = "activity")]
///     activities: Vec<String>,
/// }
///
/// // with lifetime
/// #[derive(Setter, Getter, Builder, Debug)]
/// pub struct LifetimeUser<'a> {
///     id: u32,
///     age: u8,
///     name: &'a str,
///     email: &'a str,
///     hobby: Box<&'a str>,
/// }
///
/// // ----------------------------------------------------------------
///
/// // Usage:
///
///    let user = User::builder()
///         .id(10086)
///         .age(18)
///         .name("photowey".to_string())
///         .email("photowey@gmail.com".to_string())
///         .hobby(vec!["badminton".to_string()])
///         // @since 0.2.0
///         .activities(vec!["badminton".to_string()])
///         // #[builder] function
///         .activity("badminton".to_string())
///         .build()
///         .unwrap();
///
///     assert_eq!(&10086u32, user.get_id());
///     assert_eq!(&18u8, user.get_age());
///
///     assert_eq!("photowey", user.get_name());
///     assert_eq!("photowey@gmail.com", user.get_email());
///     assert_eq!(&vec!["badminton".to_string()], user.get_hobby());
///
/// // ----------------------------------------------------------------
///
///     let rvt = User::builder()
///         //.id(10086)
///         .age(18)
///         .name("photowey".to_string())
///         .email("photowey@gmail.com".to_string())
///         .hobby(vec!["badminton".to_string()])
///         // @since 0.2.0
///         .activities(vec!["badminton".to_string()])
///         // #[builder] function
///         .activity("badminton".to_string())
///         .build();
///
///     // Missing field: `id`!
///     assert!(rvt.is_err())
/// ```
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    derive_builder(input)
}

/// `Data` is a composite macro that includes [`Setter`], [`Getter`], and [`Builder`].
#[proc_macro_derive(Data)]
pub fn data_derive(input: TokenStream) -> TokenStream {
    TokenStream::from_iter(
        vec![
            derive_setter(input.clone()),
            derive_getter(input.clone()),
            derive_builder(input.clone()),
        ]
        .into_iter(),
    )
}
