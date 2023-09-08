//
#[macro_export]
macro_rules! mssql_executor_alias {
    (
        $pub:vis $name:ident
    ) => {
        $pub trait $name<'c>: sqlx::MssqlExecutor<'c> {}
        impl<'c, T: sqlx::MssqlExecutor<'c>> $name<'c> for T {}
    };
}

#[macro_export]
macro_rules! mysql_executor_alias {
    (
        $pub:vis $name:ident
    ) => {
        $pub trait $name<'c>: sqlx::MySqlExecutor<'c> {}
        impl<'c, T: sqlx::MySqlExecutor<'c>> $name<'c> for T {}
    };
}

#[macro_export]
macro_rules! postgres_executor_alias {
    (
        $pub:vis $name:ident
    ) => {
        $pub trait $name<'c>: sqlx::PgExecutor<'c> {}
        impl<'c, T: sqlx::PgExecutor<'c>> $name<'c> for T {}
    };
}

#[macro_export]
macro_rules! sqlite_executor_alias {
    (
        $pub:vis $name:ident
    ) => {
        $pub trait $name<'c>: sqlx::SqliteExecutor<'c> {}
        impl<'c, T: sqlx::SqliteExecutor<'c>> $name<'c> for T {}
    };
}

#[cfg(test)]
mod tests {

    crate::postgres_executor_alias!(pub FooPgExecutor);
    crate::sqlite_executor_alias!(pub FooSqliteExecutor);

    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn simple() {
        //
        async fn postgres_execute<'c, E: FooPgExecutor<'c>>(executor: E) {
            let _ = sqlx::query("select 1").execute(executor).await;
        }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://127.0.0.1:5432")
            .expect("");
        match timeout(Duration::from_millis(500), postgres_execute(&pool)).await {
            Ok(_) => {}
            Err(_) => {
                eprintln!("timeout")
            }
        }

        match timeout(Duration::from_millis(500), pool.acquire()).await {
            Ok(Ok(mut conn)) => {
                match timeout(Duration::from_millis(500), postgres_execute(conn.as_mut())).await {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("timeout")
                    }
                }
            }
            Ok(Err(_)) => {
                eprintln!("acquire failed")
            }
            Err(_) => {
                eprintln!("timeout")
            }
        }

        //
        #[allow(dead_code)]
        async fn sqlite_execute<'c, E: FooSqliteExecutor<'c>>(executor: E) {
            let _ = sqlx::query("select 1").execute(executor).await;
        }
    }
}

/*
// Cannot be used in Transaction.
pub trait FooPgExecutorOld: for<'c> sqlx::PgExecutor<'c> {}
impl<T: for<'c> sqlx::PgExecutor<'c>> FooPgExecutorOld for T {}
*/
