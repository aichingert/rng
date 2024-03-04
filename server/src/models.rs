use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

use crate::schema::users;


#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn create(
        conn: &mut PgConnection, 
        user: NewUser,
    ) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
        users::dsl::users
            .select(User::as_select())
            .filter(users::dsl::username.eq(username))
            .first(conn)
            .ok()
    }
}
