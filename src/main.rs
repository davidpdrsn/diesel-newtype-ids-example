#[macro_use]
extern crate diesel;

use diesel::backend::{self, Backend};
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::expression::Expression;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::HasSqlType;
use diesel::sql_types::Integer;
use diesel::sql_types::TypeMetadata;

mod schema {
    table! {
        use diesel::sql_types::*;
        use crate::UserId;

        users (id) {
            id -> UserId,
            name -> Text,
            age -> Integer,
            country_id -> Nullable<Integer>,
            home_city_id -> Nullable<Integer>,
            current_city_id -> Nullable<Integer>,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct UserId(i32);

impl HasSqlType<UserId> for Pg
where
    Pg: TypeMetadata + HasSqlType<Integer>,
{
    fn metadata(lookup: &Self::MetadataLookup) -> <Pg as TypeMetadata>::TypeMetadata {
        <Pg as HasSqlType<Integer>>::metadata(lookup)
    }
}

impl<DB> FromSql<UserId, DB> for i32
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        i32::from_sql(bytes)
    }
}

impl<ST, DB> Queryable<ST, DB> for UserId
where
    DB: Backend,
    i32: Queryable<ST, DB>,
{
    type Row = <i32 as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        UserId(i32::build(row))
    }
}

#[derive(Queryable, Debug, Eq, PartialEq)]
struct User {
    id: UserId,
    name: String,
    age: i32,
    country_id: Option<i32>,
    home_city_id: Option<i32>,
    current_city_id: Option<i32>,
}

fn main() {
    use schema::users;

    let con = establish_connection();

    diesel::insert_into(users::table)
        .values((users::name.eq("Bob"), users::age.eq(30)))
        .execute(&con)
        .unwrap();

    let users = users::table
        .select(users::all_columns)
        .load::<User>(&con)
        .unwrap();

    assert_eq!(1, users.len());

    // let user = users::table
    //     .filter(users::id.eq(users[0].id))
    //     .first::<User>(&con)
    //     .unwrap();

    // assert_eq!(vec![user], users);
}

fn establish_connection() -> PgConnection {
    let database_url = "postgres://localhost/diesel_factories_test";
    let con = PgConnection::establish(&database_url).unwrap();
    con.begin_test_transaction().unwrap();
    con
}
