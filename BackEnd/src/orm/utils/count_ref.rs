//! CountReferencesDsl: a DSL for counting references in a table
//! `one.count_references_in(many)` is equivalent to `SELECT COUNT(*) FROM many_table WHERE many = one`

use diesel::dsl::{count_star, CountStar, Eq};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::sql_types::{SingleValue, SqlType};

pub trait CountReferencesDsl<V> {
    type Output;
    fn count_references_in(self, many: V) -> Self::Output;
}

impl<T, U, V, S, R1, R2, R> CountReferencesDsl<V> for U
where
    U: Expression + AsExpression<S>,
    T: Table<Query = R1> + Default,
    V: Column<Table = T> + Expression<SqlType = S>,
    S: SqlType + SingleValue,
    R1: SelectDsl<CountStar, Output = R2>,
    R2: FilterDsl<Eq<V, U>, Output = R>,
{
    type Output = R;
    fn count_references_in(self, many: V) -> Self::Output {
        SelectDsl::select(V::Table::default(), count_star()).filter(many.eq(self))
    }
}

pub type CountReferencesIn<U, V> = <U as CountReferencesDsl<V>>::Output;
