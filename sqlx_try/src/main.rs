
use sqlx::postgres::PgPoolOptions;

struct Todos {
    id : i64,
    description : String,
    done : bool
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), sqlx::Error> {
    // let database_url = "postgres://postgres:mysecretpassword@0.0.0.0:5432/steotest?schema=public";
    let database_url = "postgres://postgres:mysecretpassword@0.0.0.0:5432/steotest?schema=public";
    let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url).await?;

    let recs = sqlx::query_as!(Todos,
                "
                SELECT id, description, done
                FROM todos
                ORDER BY id")
            .fetch_all(&pool)
            .await?;

    for rec in recs {
        println!("- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}
