#[macro_export]
macro_rules! diesel_id_newtype {
    ( $name:ident, $rust_ty:ty, $sql_ty:ty ) => {
        impl diesel::query_builder::QueryId for $name {
            type QueryId = $name;
            const HAS_STATIC_QUERY_ID: bool = true;
        }

        impl diesel::sql_types::NotNull for $name {}

        impl diesel::sql_types::SingleValue for $name {}

        impl diesel::expression::AsExpression<$name> for $name {
            type Expression = diesel::expression::bound::Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl diesel::expression::AsExpression<diesel::sql_types::Nullable<$name>> for $name {
            type Expression = diesel::expression::bound::Bound<diesel::sql_types::Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a> diesel::expression::AsExpression<$name> for &'a $name {
            type Expression = diesel::expression::bound::Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a> diesel::expression::AsExpression<diesel::sql_types::Nullable<$name>> for &'a $name {
            type Expression = diesel::expression::bound::Bound<diesel::sql_types::Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a, 'b> diesel::expression::AsExpression<$name> for &'a &'b $name {
            type Expression = diesel::expression::bound::Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<'a, 'b> diesel::expression::AsExpression<diesel::sql_types::Nullable<$name>> for &'a &'b $name {
            type Expression = diesel::expression::bound::Bound<diesel::sql_types::Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                diesel::expression::bound::Bound::new(self)
            }
        }

        impl<DB: diesel::backend::Backend> diesel::serialize::ToSql<$name, DB> for $name {
            fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, DB>) -> diesel::serialize::Result {
                <$rust_ty as diesel::serialize::ToSql<$sql_ty, DB>>::to_sql(&self.0, out)
            }
        }

        impl<DB: diesel::backend::Backend> diesel::serialize::ToSql<diesel::sql_types::Nullable<$name>, DB> for $name {
            fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, DB>) -> diesel::serialize::Result {
                diesel::serialize::ToSql::<$name, DB>::to_sql(self, out)
            }
        }

        impl diesel::sql_types::HasSqlType<$name> for diesel::pg::Pg {
            fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
                lookup.lookup_type("user_id")
            }
        }

        impl diesel::deserialize::FromSqlRow<$name, diesel::pg::Pg> for $name {
            fn build_from_row<T: diesel::row::Row<diesel::pg::Pg>>(row: &mut T) -> diesel::deserialize::Result<Self> {
                diesel::deserialize::FromSql::<$name, diesel::pg::Pg>::from_sql(row.take())
            }
        }

        impl diesel::deserialize::FromSql<$name, diesel::pg::Pg> for $name {
            fn from_sql(bytes: Option<&<diesel::pg::Pg as diesel::backend::Backend>::RawValue>) -> diesel::deserialize::Result<Self> {
                let n = <$rust_ty as diesel::deserialize::FromSql<$sql_ty, diesel::pg::Pg>>::from_sql(bytes)?;
                Ok($name(n))
            }
        }

        impl diesel::deserialize::Queryable<$name, diesel::pg::Pg> for $name {
            type Row = Self;

            fn build(row: Self::Row) -> Self {
                row
            }
        }

        impl From<$name> for juniper::ID {
            fn from(value: $name) -> juniper::ID {
                juniper::ID::from(value.0.to_string())
            }
        }
    };
}
