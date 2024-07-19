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

use lombokrs::{Builder, Getter, Setter};

#[cfg(test)]
mod tests;

// ----------------------------------------------------------------

#[derive(Setter, Getter, Builder, Debug)]
pub struct User {
    id: u32,
    age: u8,
    name: String,
    email: String,
    hobby: Vec<String>,
}

#[derive(Setter, Getter, Builder, Debug)]
pub struct LifetimeUser<'a> {
    id: u32,
    age: u8,
    name: &'a str,
    email: &'a str,
    hobby: Box<&'a str>,
}

// ----------------------------------------------------------------

impl User {
    pub fn new(id: u32, age: u8, name: String, email: String, hobby: Vec<String>) -> Self {
        Self {
            id,
            age,
            name,
            email,
            hobby,
        }
    }
}

// ----------------------------------------------------------------

impl<'a> LifetimeUser<'a> {
    pub fn new(id: u32, age: u8, name: &'a str, email: &'a str, hobby: Box<&'a str>) -> Self {
        Self {
            id,
            age,
            name,
            email,
            hobby,
        }
    }
}
