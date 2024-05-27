use diesel::debug_query;
use diesel::helper_types::Limit;
use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::query_dsl::methods::LimitDsl;
use diesel_async::methods::{ExecuteDsl, LoadQuery};
use diesel_async::return_futures::{GetResult, LoadFuture};
use diesel_async::{AsyncConnection, RunQueryDsl as DieselAsyncRunQueryDsl};

pub trait RunQueryDsl<Conn>: Sized
where
    Self: QueryFragment<Pg>,
{
    fn log_query(&self, name: &str) {
        trace!("{name}: {}", debug_query::<Pg, _>(self));
    }

    fn execute<'conn, 'query>(self, conn: &'conn mut Conn) -> Conn::ExecuteFuture<'conn, 'query>
    where
        Conn: AsyncConnection + Send,
        Self: ExecuteDsl<Conn> + 'query,
    {
        self.log_query("execute");
        DieselAsyncRunQueryDsl::execute(self, conn)
    }

    fn get_result<'query, 'conn, U>(
        self,
        conn: &'conn mut Conn,
    ) -> GetResult<'conn, 'query, Self, Conn, U>
    where
        U: Send + 'conn,
        Conn: AsyncConnection,
        Self: LoadQuery<'query, Conn, U> + 'query,
    {
        self.log_query("get_result");
        DieselAsyncRunQueryDsl::get_result(self, conn)
    }

    fn get_results<'query, 'conn, U>(
        self,
        conn: &'conn mut Conn,
    ) -> LoadFuture<'conn, 'query, Self, Conn, U>
    where
        U: Send,
        Conn: AsyncConnection,
        Self: LoadQuery<'query, Conn, U> + 'query,
    {
        self.log_query("get_results");
        DieselAsyncRunQueryDsl::get_results(self, conn)
    }

    fn first<'query, 'conn, U>(
        self,
        conn: &'conn mut Conn,
    ) -> GetResult<'conn, 'query, Limit<Self>, Conn, U>
    where
        U: Send + 'conn,
        Conn: AsyncConnection,
        Self: LimitDsl,
        Limit<Self>: LoadQuery<'query, Conn, U> + Send + 'query,
    {
        self.log_query("first");
        DieselAsyncRunQueryDsl::first(self, conn)
    }
}

impl<T, Conn> RunQueryDsl<Conn> for T where T: QueryFragment<Pg> {}
