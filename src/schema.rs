table! {
    use diesel::sql_types::*;
    use crate::models::{CountryId, UserId, CityId};

    users {
        id -> UserId,
        name -> Text,
        age -> Integer,
        country_id -> Nullable<CountryId>,
        home_city_id -> Nullable<CityId>,
        current_city_id -> Nullable<CityId>,
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

table! {
    use diesel::sql_types::*;
    use crate::models::{CityId, CountryId};

    cities {
        id -> CityId,
        name -> Text,
        team_association -> Text,
        association_label -> Text,
        country_id -> CountryId,
    }
}
