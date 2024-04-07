//! Bracketed helper: a wrapper for expressions that need to be bracketed

use diesel::backend::Backend;
use diesel::dsl::Limit;
use diesel::expression::{TypedExpressionType, ValidGrouping};
use diesel::prelude::*;
use diesel::query_builder::{AstPass, QueryFragment, QueryId, SelectQuery};
use diesel::query_dsl::methods::LimitDsl;

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
