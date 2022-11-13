//! Derive macro to generate SQL statements for data structures.
//!
//! # Quick start
//!
//! Add `worm-rust` dependency to your `Cargo.toml`.
//! ```toml
//! [dependencies]
//! worm-rust = "0.1.0"
//! ```
//!
//! ## Add the macro to your structure
//!
//! ```rust
//! use worm_rust::WormTable;
//!
//! #[derive(WormTable)]
//! struct Person {
//!   #[text(primary = true)]
//!   id: u32,
//!   #[text(null = true)]
//!   name: String,
//!   #[integer()]
//!   age: u32,
//! }
//! ```
//!
//! ## What you can use
//!
//! ```rust
//! use rusqlite;
//! use worm_rust::WormTable;
//!
//! #[derive(WormTable)]
//! struct Person {
//!   #[integer(primary = true)]
//!   id: u32,
//!   #[text(null = true)]
//!   name: String,
//!   #[integer()]
//!   age: u32,
//! }
//!
//!
//! let conn = rusqlite::Connection::open_in_memory().unwrap();
//! let person_sql = Person::from_rusqlite(&conn);
//!
//! // Create Table in SQL database
//! person_sql.create_table().unwrap();
//!
//! // Insert person into SQL database
//! let person = Person { id: 1, name: "John".to_string(), age: 24 };
//! person_sql.insert(&person).unwrap();
//!
//! // Retrieve list of persons from SQL database
//! assert!(person_sql.count_all().unwrap() == 1);
//! let persons: Vec<Person> = person_sql.select_all().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("John"));
//!
//! // Insert Jim
//! let jim = Person { id: 2, name: "Jim".to_string(), age: 27 };
//! person_sql.insert(&jim).unwrap();
//!
//! // Check Jim's age
//! let select = SelectBuilder::default().set_filter(Filter::NameEqual("Jim".to_string())).build();
//! let p: Person = person_sql.select(select).unwrap().into_iter().nth(0).unwrap();
//! assert!(p.age == 27);
//! // or
//! let filter = Filter::And(Box::new(Filter::NameEqual("Jim".to_string())), Box::new(Filter::AgeEqual(27)));
//! assert!(person_sql.count(filter.into()).unwrap() == 1);
//!
//! // Update Jim
//! let filter = Filter::NameEqual("Jim".to_string());
//! let jim: Person = person_sql.select_one(filter.into()).unwrap().unwrap();
//! let update_to = Person { id: 2, name: jim.name.clone(), age: jim.age+1 };
//! person_sql.update_to(&jim, &update_to).unwrap();
//!
//! let filter = Filter::NameEqual("Jim".to_string());
//! let updated_jane: Person = person_sql.select(filter.into()).unwrap().into_iter().nth(0).unwrap();
//! assert!(updated_jane.age == 28);
//!
//! // Check Jim's age
//! let p: Person = person_sql.select(Filter::AgeGreaterThan(27).into()).unwrap().into_iter().nth(0).unwrap();
//! assert!(p.age == 28);
//!
//! // Delete John
//! let filter = Filter::And(Box::new(Filter::NameEqual("John".to_string())), Box::new(Filter::AgeEqual(24)));
//! let john: Person = person_sql.select(filter.into()).unwrap()
//!             .into_iter().nth(0).unwrap();
//! person_sql.delete(john.into()).unwrap();
//!
//! let filter = Filter::NameEqual("John".to_string());
//! let john: Vec<Person> = person_sql.select(filter.into()).unwrap();
//! assert!(john.len() == 0);
//!
//! // Check that database only contains Jane
//! let persons: Vec<Person> = person_sql.select_all().unwrap();
//! assert!(persons.len() == 1);
//! assert!(persons[0].name.eq("Jim"));
//!
//! ```

pub use worm_rust_macro::*;

#[allow(dead_code)]
fn nothing_here() {
	todo!()
}
