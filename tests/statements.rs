use worm;

#[derive(worm::Table)]
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
	assert_eq!(Users::create_table(), "CREATE TABLE IF NOT EXISTS Users ( user_id INTEGER NOT NULL PRIMARY KEY, user_name VARCHAR(120) NOT NULL UNIQUE, first_name VARCHAR(30), last_name VARCHAR(30), note TEXT, birthday INTEGER NULL )");
	assert_eq!(Users::delete_table(), "DROP TABLE Users");
	
	assert_eq!(Users::insert(), "INSERT INTO Users (user_id, user_name, first_name, last_name, note, birthday) VALUES (?1, ?2, ?3, ?4, ?5, ?6)");
	assert_eq!(Users::update_by_id(), "UPDATE Users SET user_id = ?2, user_name = ?3, first_name = ?4, last_name = ?5, note = ?6, birthday = ?7 WHERE user_id = ?1");
	
	let filter = Filter::UserIdEqual(1);
	assert_eq!(Users::select(filter.clone().into()), "SELECT user_id, user_name, first_name, last_name, note, birthday FROM Users WHERE user_id = 1");
	assert_eq!(
		Users::delete(filter.clone().into()),
		"DELETE FROM Users WHERE user_id = 1"
	);
	
	let filter = Filter::And(
		Box::new(Filter::UserIdEqual(1)),
		Box::new(Filter::LastNameEqual(String::from("Dane"))),
	);
	assert_eq!(Users::select(filter.clone().into()), "SELECT user_id, user_name, first_name, last_name, note, birthday FROM Users WHERE (user_id = 1 AND last_name = 'Dane')");
	assert_eq!(
		Users::delete(filter.clone().into()),
		"DELETE FROM Users WHERE (user_id = 1 AND last_name = 'Dane')"
	);
	
	assert_eq!(
		Users::select_all(),
		"SELECT user_id, user_name, first_name, last_name, note, birthday FROM Users"
	);
	assert_eq!(Users::delete_all(), "DELETE FROM Users");
}
