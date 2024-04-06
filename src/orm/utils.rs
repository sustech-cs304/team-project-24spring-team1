use diesel::backend::Backend;
use diesel::dsl::{count_star, CountStar, Eq, Limit};
use diesel::expression::{AsExpression, TypedExpressionType, ValidGrouping};
use diesel::prelude::*;
use diesel::query_builder::{AstPass, QueryFragment, QueryId, SelectQuery};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl};
use diesel::sql_types::{SingleValue, SqlType};

/// CountReferencesDsl: a DSL for counting references in a table
/// `one.count_references_in(many)` is equivalent to `SELECT COUNT(*) FROM many_table WHERE many = one`

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

/// Bracketed helper: a wrapper for expressions that need to be bracketed

#[derive(QueryId)]
pub struct Bracketed<T>(T);

pub trait BracketDsl {
    type Output;
    fn bracket(self) -> Bracketed<Self::Output>;
}

pub type Bracket<T> = Bracketed<<T as BracketDsl>::Output>;

impl<T: SelectQuery + LimitDsl> BracketDsl for T {
    type Output = Limit<Self>;
    fn bracket(self) -> Bracketed<Self::Output> {
        Bracketed(self.limit(1))
    }
}

impl<T: SelectQuery> Expression for Bracketed<T>
where
    T::SqlType: TypedExpressionType,
{
    type SqlType = T::SqlType;
}

impl<DB: Backend, T: QueryFragment<DB>> QueryFragment<DB> for Bracketed<T> {
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        out.push_sql("(");
        self.0.walk_ast(out.reborrow())?;
        out.push_sql(")");
        Ok(())
    }
}

impl<T> ValidGrouping<()> for Bracketed<T> {
    type IsAggregate = diesel::expression::is_aggregate::No;
}

impl<T, QS> SelectableExpression<QS> for Bracketed<T> where Bracketed<T>: AppearsOnTable<QS> {}
impl<T, QS> AppearsOnTable<QS> for Bracketed<T> where Bracketed<T>: Expression {}
