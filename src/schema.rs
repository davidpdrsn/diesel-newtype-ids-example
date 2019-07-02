table! {
    use diesel::sql_types::*;
    use crate::models::{CountryId, UserId};

    users {
        id -> UserId,
        name -> Text,
        age -> Integer,
        country_id -> Nullable<CountryId>,
        home_city_id -> Nullable<Integer>,
        current_city_id -> Nullable<Integer>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::CountryId;

    countries {
        id -> CountryId,
        name -> Text,
    }
}
