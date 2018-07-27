use actix_web::actix::*;
use diesel::helper_types::Limit;
use diesel::query_dsl::methods::{ExecuteDsl, LimitDsl, LoadQuery};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use failure::Error;
use r2d2::{self, PooledConnection};
use std::marker::PhantomData;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DbExecutor(pub Pool);

pub struct Execute<T>(pub T);

pub struct Load<T, U>(pub T, PhantomData<U>);
impl<T, U> Load<T, U> {
    pub fn new(n: T) -> Self {
        Load(n, PhantomData)
    }
}
pub struct GetResult<T, U>(pub T, PhantomData<U>);
impl<T, U> GetResult<T, U> {
    pub fn new(n: T) -> Self {
        GetResult(n, PhantomData)
    }
}
pub struct First<T, U>(pub T, PhantomData<U>);
impl<T, U> First<T, U> {
    pub fn new(n: T) -> Self {
        First(n, PhantomData)
    }
}

impl<T> Message for Execute<T> {
    type Result = Result<usize, Error>;
}
impl<T, U: 'static> Message for Load<T, U> {
    type Result = Result<Vec<U>, Error>;
}

impl<T, U: 'static> Message for GetResult<T, U> {
    type Result = Result<U, Error>;
}

impl<T, U: 'static> Message for First<T, U> {
    type Result = Result<U, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl<T> Handler<Execute<T>> for DbExecutor
where
    T: RunQueryDsl<Connection> + ExecuteDsl<Connection>,
{
    type Result = <Execute<T> as Message>::Result;

    fn handle(&mut self, Execute(msg): Execute<T>, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        msg.execute(&conn).map_err(From::from)
    }
}

impl<T, U: 'static> Handler<Load<T, U>> for DbExecutor
where
    T: RunQueryDsl<Connection> + LoadQuery<Connection, U>,
{
    type Result = <Load<T, U> as Message>::Result;

    fn handle(&mut self, Load(msg, _): Load<T, U>, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        msg.load(&conn).map_err(From::from)
    }
}

impl<T, U: 'static> Handler<GetResult<T, U>> for DbExecutor
where
    T: RunQueryDsl<Connection> + LoadQuery<Connection, U>,
{
    type Result = <GetResult<T, U> as Message>::Result;

    fn handle(
        &mut self,
        GetResult(msg, _): GetResult<T, U>,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = self.0.get()?;
        msg.get_result::<U>(&conn).map_err(From::from)
    }
}

impl<T, U: 'static> Handler<First<T, U>> for DbExecutor
where
    T: LimitDsl + RunQueryDsl<Connection>,
    Limit<T>: LoadQuery<Connection, U>,
{
    type Result = <First<T, U> as Message>::Result;

    fn handle(&mut self, First(msg, _): First<T, U>, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        msg.first(&conn).map_err(From::from)
    }
}
