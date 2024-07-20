# `lombokrs`

`lombokrs`(`Lombok` Rust) is a lightweight Rust macro library. It is the simple implementation of `Lombok` Java in Rust.

## Acknowledgment

This project, `lombokrs`, was developed with significant inspiration from the open-source project `lombok-rs`. Special
thanks to the contributors of [lombok-rs](https://github.com/sokomishalov/lombok-rs) for their excellent work,
particularly in handling `Lifetimes`. While
building upon their foundation, I have made several modifications based on my understanding, especially in the areas of
Builder and Getter implementations.

## 2. Implementation

- [x] `@Getter` - `#[derive(Getter)]`
- [x] `@Setter` - `#[derive(Setter)]`
- [x] `@Builder` - `#[derive(Builder)]`
- [x] `@Data` - `#[derive(Data)]`
- [ ] --
- [ ] `@EqualsAndHashCode` - `#[derive(EqualsAndHashCode)]`
- [ ] `@ToString` - `#[derive(ToString)]`
- [ ] `@Value` - `#[derive(Value)]`
- [ ] `@NoArgsConstructor` - `#[derive(NoArgsConstructor)]`
- [ ] `@AllArgsConstructor` - `#[derive(AllArgsConstructor)]`

### 2.1. Explanation

Why the annotations below are not implemented:

- `EqualsAndHashCode`
- `ToString`
- `Value`
- `NoArgsConstructor`
- `AllArgsConstructor`

1. In the actual development process, `Equals`, `ToString`, `HashCode`, `Value` etc. are not used very often;
2. `NoArgsConstructor` can be replaced by `Default` Trait;
3. `AllArgsConstructor` can be replaced by builder mode.

Based on the above reasons, it is not implemented. If necessary, please
use  [lombok-rs](https://github.com/sokomishalov/lombok-rs) crates instead.

## 3. Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lombokrs = "0.1"
```

## 4.`APIs`

### 4.0. `Prepare`

```rust
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

#[derive(Data, Debug)]
pub struct DataUser {
    id: u32,
    age: u8,
    name: String,
    email: String,
    hobby: Vec<String>,
}

// ----------------------------------------------------------------

impl User {
    pub fn new(...) -> Self {}
}

// ----------------------------------------------------------------

impl<'a> LifetimeUser<'a> {
    pub fn new(...) -> Self {}
}
```

### 4.1. `Setter`

```rust
let mut user = User::new(
10086,
18,
"photowey".to_string(),
"photowey@gmail.com".to_string(),
vec!["badminton".to_string()],
);

// ----------------------------------------------------------------

assert_eq!(&10086u32, user.get_id());
assert_eq!(&18u8, user.get_age());

assert_eq!("photowey", user.get_name());
assert_eq!("photowey@gmail.com", user.get_email());
assert_eq!(&vec!["badminton".to_string()], user.get_hobby());

// ---------------------------------------------------------------- Setter

user.set_id(9527);
user.set_age(25);
user.set_name("lombokrs".to_string());
user.set_email("lombokrs@gmail.com".to_string());
user.set_hobby(vec!["football".to_string()]);

// ----------------------------------------------------------------

assert_eq!(&9527u32, user.get_id());
assert_eq!(&25u8, user.get_age());

assert_eq!("lombokrs", user.get_name());
assert_eq!("lombokrs@gmail.com", user.get_email());
assert_eq!(&vec!["football".to_string()], user.get_hobby());
```

### 4.2. `Getter`

```rust
let user = User::new(
10086,
18,
"photowey".to_string(),
"photowey@gmail.com".to_string(),
vec!["badminton".to_string()],
);

// ---------------------------------------------------------------- Getter | get_x()

assert_eq!(&10086u32, user.get_id());
assert_eq!(&18u8, user.get_age());

assert_eq!("photowey", user.get_name());
assert_eq!("photowey@gmail.com", user.get_email());
assert_eq!(&vec!["badminton".to_string()], user.get_hobby());

// ---------------------------------------------------------------- Getter/fluent | x()

assert_eq!(&10086u32, user.id());
assert_eq!(&18u8, user.age());

assert_eq!("photowey", user.name());
assert_eq!("photowey@gmail.com", user.email());
assert_eq!(&vec!["badminton".to_string()], user.hobby());
```

### 4.3. `Builder`

```rust
// ---------------------------------------------------------------- Builder
// UserBuilder = User::builder()

let user = User::builder()
.id(10086)
.age(18)
.name("photowey".to_string())
.email("photowey@gmail.com".to_string())
.hobby(vec!["badminton".to_string()])
.build();

// ----------------------------------------------------------------

assert_eq!(&10086u32, user.get_id());
assert_eq!(&18u8, user.get_age());

assert_eq!("photowey", user.get_name());
assert_eq!("photowey@gmail.com", user.get_email());
assert_eq!(&vec!["badminton".to_string()], user.get_hobby());
```

### 4.4. `Data`

```rust
// ---------------------------------------------------------------- Builder

let mut user = DataUser::builder()
.id(10086)
.age(18)
.name("photowey".to_string())
.email("photowey@gmail.com".to_string())
.hobby(vec!["badminton".to_string()])
.build();

// ---------------------------------------------------------------- Setter

user.set_id(9527);
user.set_age(25);
user.set_name("lombokrs".to_string());
user.set_email("lombokrs@gmail.com".to_string());
user.set_hobby(vec!["football".to_string()]);

// ---------------------------------------------------------------- Getter | get_x()

assert_eq!(&9527u32, user.get_id());
assert_eq!(&25u8, user.get_age());

assert_eq!("lombokrs", user.get_name());
assert_eq!("lombokrs@gmail.com", user.get_email());
assert_eq!(&vec!["football".to_string()], user.get_hobby());

// ---------------------------------------------------------------- Getter/fluent | x()

assert_eq!(&9527u32, user.id());
assert_eq!(&25u8, user.age());

assert_eq!("lombokrs", user.name());
assert_eq!("lombokrs@gmail.com", user.email());
assert_eq!(&vec!["football".to_string()], user.hobby());
```

