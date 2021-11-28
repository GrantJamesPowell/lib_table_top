#![allow(dead_code)]
use sqlx::migrate::MigrateError;
use sqlx::SqlitePool;

async fn migrate(pool: &SqlitePool) -> Result<(), MigrateError> {
    sqlx::migrate!("db/migrations").run(pool).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn open_db() {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .connect("sqlite::memory:")
            .await
            .expect("can open");

        migrate(&pool).await.expect("Could Migrate");

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await
            .expect("can query");

        assert_eq!(row.0, 150);

        #[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
        struct User {
            first_name: String,
            last_name: String,
        }

        let gp = User {
            first_name: String::from("grant"),
            last_name: String::from("powell"),
        };

        sqlx::query("INSERT INTO peeps (first_name, last_name) VALUES ($1, $2)")
            .bind(&gp.first_name)
            .bind(&gp.last_name)
            .execute(&pool)
            .await
            .expect("VALID");

        let res: sqlx::Result<Vec<User>> = sqlx::query_as("SELECT * FROM peeps;")
            .fetch_all(&pool)
            .await;

        assert_eq!(res.unwrap(), vec![gp]);
    }
}
