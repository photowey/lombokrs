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

// tests

// ----------------------------------------------------------------

use crate::User;

#[test]
fn test_setter() {
    let mut user = User::new(
        10086,
        18,
        "photowey".to_string(),
        "photowey@gmail.com".to_string(),
        vec!["badminton".to_string()],
    );

    assert_eq!(&10086u32, user.get_id());
    assert_eq!(&18u8, user.get_age());

    assert_eq!("photowey", user.get_name());
    assert_eq!("photowey@gmail.com", user.get_email());
    assert_eq!(&vec!["badminton".to_string()], user.get_hobby());

    // ----------------------------------------------------------------

    user.set_id(9527);
    user.set_age(25);
    user.set_name("lombokrs".to_string());
    user.set_email("lombokrs@gmail.com".to_string());
    user.set_hobby(vec!["football".to_string()]);

    assert_eq!(&9527u32, user.get_id());
    assert_eq!(&25u8, user.get_age());

    assert_eq!("lombokrs", user.get_name());
    assert_eq!("lombokrs@gmail.com", user.get_email());
    assert_eq!(&vec!["football".to_string()], user.get_hobby());
}

#[test]
fn test_getter() {
    let user = User::new(
        10086,
        18,
        "photowey".to_string(),
        "photowey@gmail.com".to_string(),
        vec!["badminton".to_string()],
    );

    assert_eq!(&10086u32, user.get_id());
    assert_eq!(&18u8, user.get_age());

    assert_eq!("photowey", user.get_name());
    assert_eq!("photowey@gmail.com", user.get_email());
    assert_eq!(&vec!["badminton".to_string()], user.get_hobby());
}

#[test]
fn test_builder() {
    let user = User::builder()
        .id(10086)
        .age(18)
        .name("photowey".to_string())
        .email("photowey@gmail.com".to_string())
        .hobby(vec!["badminton".to_string()])
        .build();

    assert_eq!(&10086u32, user.get_id());
    assert_eq!(&18u8, user.get_age());

    assert_eq!("photowey", user.get_name());
    assert_eq!("photowey@gmail.com", user.get_email());
    assert_eq!(&vec!["badminton".to_string()], user.get_hobby());
}

#[test]
fn test_builder_default() {
    let user = User::builder().build();

    assert_eq!(&0u32, user.get_id());
    assert_eq!(&0u8, user.get_age());

    assert_eq!("", user.get_name());
    assert_eq!("", user.get_email());
    assert_eq!(&Vec::<String>::new(), user.get_hobby());
}
