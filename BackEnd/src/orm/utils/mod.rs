use diesel::associations::HasTable;
use diesel::query_builder::{IntoUpdateTarget, UpdateStatement};

mod bracket;
mod count_ref;
mod functions;
mod logging;
pub mod sql_types;
pub mod types;

pub use bracket::{Bracket, BracketDsl};
pub use count_ref::{CountReferencesDsl, CountReferencesIn};
pub use functions::coalesce;
pub use logging::RunQueryDsl;

pub type Update<Target> =
    UpdateStatement<<Target as HasTable>::Table, <Target as IntoUpdateTarget>::WhereClause>;
