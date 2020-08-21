use super::schema::users;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub picture: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub email: Option<&'a str>,
}
