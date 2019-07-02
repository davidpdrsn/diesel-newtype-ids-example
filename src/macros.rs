#[macro_export]
macro_rules! diesel_id_newtype {
    ( $name:ident ) => {
        impl QueryId for $name {
            type QueryId = $name;
            const HAS_STATIC_QUERY_ID: bool = true;
        }

        impl NotNull for $name {}

        impl SingleValue for $name {}

        impl AsExpression<$name> for $name {
            type Expression = Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl AsExpression<Nullable<$name>> for $name {
            type Expression = Bound<Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl<'a> AsExpression<$name> for &'a $name {
            type Expression = Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl<'a> AsExpression<Nullable<$name>> for &'a $name {
            type Expression = Bound<Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl<'a, 'b> AsExpression<$name> for &'a &'b $name {
            type Expression = Bound<$name, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl<'a, 'b> AsExpression<Nullable<$name>> for &'a &'b $name {
            type Expression = Bound<Nullable<$name>, Self>;

            fn as_expression(self) -> Self::Expression {
                Bound::new(self)
            }
        }

        impl<DB: Backend> ToSql<$name, DB> for $name {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                <i32 as ToSql<Integer, DB>>::to_sql(&self.0, out)
            }
        }

        impl<DB: Backend> ToSql<Nullable<$name>, DB> for $name {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
                ToSql::<$name, DB>::to_sql(self, out)
            }
        }

        impl HasSqlType<$name> for Pg {
            fn metadata(lookup: &Self::MetadataLookup) -> Self::TypeMetadata {
                lookup.lookup_type("user_id")
            }
        }

        impl FromSqlRow<$name, Pg> for $name {
            fn build_from_row<T: Row<Pg>>(row: &mut T) -> deserialize::Result<Self> {
                FromSql::<$name, Pg>::from_sql(row.take())
            }
        }

        impl FromSql<$name, Pg> for $name {
            fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
                let n = <i32 as FromSql<_, Pg>>::from_sql(bytes)?;
                Ok($name(n))
            }
        }

        impl Queryable<$name, Pg> for $name {
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
