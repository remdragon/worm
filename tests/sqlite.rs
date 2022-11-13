use std::error::Error;
use worm_rust::WormTable;

#[derive(WormTable, Clone)]
pub struct Users {
	#[integer(primary = true)]
	user_id: u32,
	#[varchar(size = 120, unique = true, null = false)]
	user_name: String,
	#[varchar(size = 30)]
	first_name: String,
	#[varchar(size = 30)]
	last_name: String,
	#[text()]
	note: String,
	#[integer(null = true)]
	birthday: u32,
}

#[test]
fn main() {
	let conn = rusqlite::Connection::open_in_memory().unwrap();
	if let Err(e) = test(&conn) {
		println!("contact::sample failed with error: {}", e);
		std::process::exit(1);
	}
}

fn test(conn: &rusqlite::Connection) -> Result<(), Box<dyn Error>> {
	let db = Users::from_rusqlite(conn);
	db.create_table()?;
	
	let birthday = 214014012;
	let contact = Users {
		user_id: 4,
		user_name: String::from("John"),
		first_name: String::from("John"),
		last_name: String::from("Bin"),
		note: String::from("This is a note"),
		birthday,
	};
	db.insert(&contact)?;
	
	db.insert(&Users {
		user_id: 5,
		user_name: String::from("Jane"),
		first_name: String::from("Jane"),
		last_name: String::from("Bin"),
		note: String::from("This is a note"),
		birthday: 214014014,
	})?;
	
	let contact = db
		.select_all()?
		.into_iter()
		.find(|c| c.user_name.eq("John"))
		.ok_or("Unable to find John Doe's contact")?;
	assert_eq!(contact.birthday, birthday);
	
	let birthday = 123;
	// Update contact
	let update = Users {
		birthday,
		..(contact.clone())
	};
	db.update_to(&contact, &update)?;
	assert!(db.count_all()? == 2);
	
	let contact = db
		.select_all()?
		.into_iter()
		.find(|c| c.user_name.eq("John"))
		.ok_or("Unable to find John Doe's contact")?;
	assert_eq!(contact.birthday, birthday);
	
	assert_eq!(db.count_all()?, 2);
	
	// Empty all contacts
	db.delete_all()?;
	assert_eq!(db.count_all()?, 0);
	
	Ok(())
}
