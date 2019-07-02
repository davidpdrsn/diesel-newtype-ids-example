#[macro_use]
extern crate diesel;

#[macro_use]
mod macros;
mod models;
mod schema;

use diesel::prelude::*;
use juniper::ID;
use juniper_eager_loading::{prelude::*, EagerLoading, HasMany, OptionHasOne};
use juniper_from_schema::graphql_schema;
use schema::*;

graphql_schema! {
    schema {
        query: Query
        mutation: Mutation
    }

    type Query {
        users: [User!]! @juniper(ownership: "owned")
    }

    type Mutation {
        noop: Boolean!
    }

    type User {
        id: ID! @juniper(ownership: "owned")
        country: Country
    }

    type Country {
        id: ID! @juniper(ownership: "owned")
        cities: [City!]!
    }

    type City {
        id: ID! @juniper(ownership: "owned")
    }
}

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_users(
        &self,
        _: &juniper::Executor<'_, Context>,
        trail: &QueryTrail<'_, User, Walked>,
    ) -> juniper::FieldResult<Vec<User>> {
        let con = establish_connection();

        let user_models = users::table.load::<models::User>(&con).unwrap();
        let mut users = User::from_db_models(&user_models);
        User::eager_load_all_children_for_each(&mut users, &user_models, &con, trail)?;

        Ok(users)
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, _: &juniper::Executor<'_, Context>) -> juniper::FieldResult<&bool> {
        Ok(&true)
    }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(
    connection = "PgConnection",
    error = "diesel::result::Error",
    id = "models::UserId"
)]
pub struct User {
    user: models::User,

    #[option_has_one(
        foreign_key_field = "country_id",
        root_model_field = "country",
        graphql_field = "country"
    )]
    country: OptionHasOne<Country>,
}

impl UserFields for User {
    fn field_id(&self, _: &juniper::Executor<'_, Context>) -> juniper::FieldResult<ID> {
        Ok(ID::from(self.user.id))
    }

    fn field_country(
        &self,
        _: &juniper::Executor<'_, Context>,
        _: &QueryTrail<'_, Country, Walked>,
    ) -> juniper::FieldResult<&Option<Country>> {
        Ok(self.country.try_unwrap()?)
    }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(
    connection = "PgConnection",
    error = "diesel::result::Error",
    id = "models::CountryId"
)]
pub struct Country {
    country: models::Country,

    #[has_many(root_model_field = "city")]
    cities: HasMany<City>,
}

impl CountryFields for Country {
    fn field_cities(
        &self,
        _: &juniper::Executor<'_, Context>,
        _: &QueryTrail<'_, City, Walked>,
    ) -> juniper::FieldResult<&Vec<City>> {
        Ok(self.cities.try_unwrap()?)
    }

    fn field_id(&self, _: &juniper::Executor<'_, Context>) -> juniper::FieldResult<ID> {
        Ok(ID::from(self.country.id))
    }
}

#[derive(Clone, EagerLoading)]
#[eager_loading(
    connection = "PgConnection",
    error = "diesel::result::Error",
    id = "models::CityId"
)]
pub struct City {
    city: models::City,
}

impl CityFields for City {
    fn field_id(&self, _: &juniper::Executor<'_, Context>) -> juniper::FieldResult<ID> {
        Ok(ID::from(self.city.id))
    }
}

fn establish_connection() -> PgConnection {
    let database_url = "postgres://localhost/diesel_factories_test";
    let con = PgConnection::establish(&database_url).unwrap();
    con.begin_test_transaction().unwrap();
    con
}

fn main() {}
