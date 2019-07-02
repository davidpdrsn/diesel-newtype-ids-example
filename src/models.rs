use crate::schema::*;
use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::Queryable;
use diesel::expression::bound::Bound;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::QueryId;
use diesel::row::Row;
use diesel::serialize::Output;
use diesel::serialize::{self, ToSql};
use diesel::sql_types::HasSqlType;
use diesel::sql_types::Integer;
use diesel::sql_types::NotNull;
use diesel::sql_types::Nullable;
use diesel::sql_types::SingleValue;
use std::io::Write;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserId(i32);

diesel_id_newtype!(UserId);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CountryId(i32);

diesel_id_newtype!(CountryId);

#[derive(Queryable, Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub age: i32,
    pub country_id: Option<CountryId>,
    pub home_city_id: Option<i32>,
    pub current_city_id: Option<i32>,
}

#[derive(Queryable, Clone, Debug, Eq, PartialEq)]
pub struct Country {
    pub id: CountryId,
    pub name: String,
}

juniper_eager_loading::impl_load_from_for_diesel! {
    (
        error = diesel::result::Error,
        connection = PgConnection,
    ) => {
        CountryId -> (countries, Country),
        UserId -> (users, User),
    }
}
