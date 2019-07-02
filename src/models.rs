use crate::schema::*;
use diesel::prelude::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UserId(i32);

diesel_id_newtype!(UserId, i32, diesel::sql_types::Integer);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CountryId(i32);

diesel_id_newtype!(CountryId, i32, diesel::sql_types::Integer);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct CityId(i64);

diesel_id_newtype!(CityId, i64, diesel::sql_types::BigInt);

#[derive(Queryable, Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub age: i32,
    pub country_id: Option<CountryId>,
    pub home_city_id: Option<CityId>,
    pub current_city_id: Option<CityId>,
}

#[derive(Queryable, Clone, Debug, Eq, PartialEq)]
pub struct Country {
    pub id: CountryId,
    pub name: String,
}

#[derive(Queryable, Clone)]
pub struct City {
    pub id: CityId,
    pub name: String,
    pub team_association: String,
    pub association_label: String,
    pub country_id: CountryId,
}


juniper_eager_loading::impl_load_from_for_diesel! {
    (
        error = diesel::result::Error,
        connection = PgConnection,
    ) => {
        CountryId -> (countries, Country),
        UserId -> (users, User),
        CityId -> (cities, City),
        Country.id -> (cities.country_id, City),
    }
}
