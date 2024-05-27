use diesel::sql_function;
use diesel::sql_types::{Nullable, SingleValue};

sql_function! { fn coalesce<T: SingleValue>(x: Nullable<T>, y: T) -> T; }
