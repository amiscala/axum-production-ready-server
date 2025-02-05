use crate::domain::models::common::string_to_sha_256;
use crate::domain::models::queries::{Transactions, VecQueries};
use crate::domain::{AppErrors, Queries};
use chrono::Utc;
use sqlx::{query_as, FromRow, Pool, Postgres};

#[derive(Clone)]
pub enum Database {
    Postgres(Pool<Postgres>),
}

pub async fn execute_vec_query<T>(
    database: &Database,
    query: VecQueries,
) -> Result<Vec<T>, AppErrors>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    match database {
        Database::Postgres(pool) => match query {
            VecQueries::GetTodos { user_id } => Ok(query_as(
                r#"
                            select * from app_todo
                            where user_id = $1
                        "#,
            )
            .bind(user_id)
            .fetch_all(pool)
            .await?),
            VecQueries::GetClients { user_id } => Ok(query_as(
                r#"
                            select * from app_client
                            where user_id = $1
                        "#,
            )
            .bind(user_id)
            .fetch_all(pool)
            .await?),
        },
    }
}

pub async fn execute_query<T>(database: &Database, query: Queries) -> Result<T, AppErrors>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    match database {
        Database::Postgres(pool) => {
            match query {
                Queries::CreateUser { user } => {
                    Ok(query_as(r#"
                            INSERT INTO app_user (user_id, email, created_at, updated_at, user_scopes, status, name, last_name)
                            VALUES ($1, $2, $3, $4, $5, $6,$7, $8)
                            RETURNING *
                        "#)
                        .bind(user.user_id)
                        .bind(user.email)
                        .bind(user.created_at)
                        .bind(user.updated_at)
                        .bind(user.user_scopes)
                        .bind(user.status)
                        .bind(user.name)
                        .bind(user.last_name)
                        .fetch_one(pool).await?)
                }
                Queries::UpdateUser { user_id, email, name, last_name } => {
                    let updated_at = Utc::now();
                    Ok(query_as(r#"
                            UPDATE app_user
                            set email = $1, name=$2, last_name=$3, updated_at = $4
                            where user_id=$5
                            RETURNING *
                        "#)
                        .bind(email)
                        .bind(name)
                        .bind(last_name)
                        .bind(updated_at)
                        .bind(user_id)
                        .fetch_one(pool).await?)
                }
                Queries::DeleteUser { user_id } => {
                    Ok(query_as(r#"
                            delete from app_user
                            where user_id = $1
                            returning *
                        "#)
                        .bind(user_id).fetch_one(pool).await?)
                }

                Queries::GetUser { user_id } => {
                    Ok(query_as(r#"
                            select * from app_user
                            where user_id = $1
                        "#)
                        .bind(user_id)
                        .fetch_one(pool).await?)
                }
                Queries::CreateClient { client } => {
                    Ok(query_as(r#"
                            INSERT INTO app_client (client_id, user_id,client_description, client_secret, client_scopes, created_at, updated_at, expires_at, status)
                            VALUES ($1, $2, $3, $4, $5, $6,$7, $8, $9)
                            RETURNING *
                        "#)
                        .bind(client.client_id)
                        .bind(client.user_id)
                        .bind(client.client_description)
                        .bind(client.client_secret)
                        .bind(client.client_scopes)
                        .bind(client.created_at)
                        .bind(client.updated_at)
                        .bind(client.expires_at)
                        .bind(client.status)
                        .fetch_one(pool).await?)
                }
                Queries::UpdateClient { user_id, client_id, client_description, client_scopes, expires_at } => {
                    let updated_at = Utc::now();
                    Ok(query_as(r#"
                            UPDATE app_client
                            set client_description = $1, client_scopes=$2, expires_at=$3, updated_at = $4
                            where user_id=$5 and client_id = $6
                            RETURNING *
                        "#)
                        .bind(client_description)
                        .bind(client_scopes)
                        .bind(expires_at)
                        .bind(updated_at)
                        .bind(user_id)
                        .bind(client_id)
                        .fetch_one(pool).await?)
                }
                Queries::DeleteClient { client_id, user_id } => {
                    Ok(query_as(r#"
                            delete from app_client
                            where user_id = $1 and client_id = $2
                            returning *
                        "#)
                        .bind(user_id)
                        .bind(client_id)
                        .fetch_one(pool).await?)
                }
                Queries::GetClient { user_id, client_id } => {
                    Ok(query_as(r#"
                            select * from app_client
                            where user_id = $1 and client_id = $2
                        "#)
                        .bind(user_id)
                        .bind(client_id)
                        .fetch_one(pool).await?)
                }
                Queries::GetClientWithClientIdAndClientSecret { client_id, client_secret } => {
                    Ok(query_as(r#"
                            select * from app_client
                            where client_id = $1 and client_secret = $2
                        "#)
                        .bind(client_id)
                        .bind(client_secret)
                        .fetch_one(pool).await?)
                }
                Queries::CreateTodo { todo } => {
                    Ok(query_as(r#"
                            INSERT INTO app_todo (todo_id, user_id,title, body, category, created_at, updated_at, status)
                            VALUES ($1, $2, $3, $4, $5, $6,$7, $8)
                            RETURNING *
                        "#)
                        .bind(todo.todo_id)
                        .bind(todo.user_id)
                        .bind(todo.title)
                        .bind(todo.body)
                        .bind(todo.category)
                        .bind(todo.created_at)
                        .bind(todo.updated_at)
                        .bind(todo.status)
                        .fetch_one(pool).await?)
                }
                Queries::UpdateTodo { user_id, todo_id, title, body, category, status } => {
                    let updated_at = Utc::now();
                    Ok(query_as(r#"
                            UPDATE app_todo
                            set title = $1, body=$2, category=$3, updated_at = $4, status = $5
                            where todo_id=$6 and user_id=$7
                            RETURNING *
                        "#)
                        .bind(title)
                        .bind(body)
                        .bind(category)
                        .bind(updated_at)
                        .bind(status)
                        .bind(todo_id)
                        .bind(user_id)
                        .fetch_one(pool).await?)
                }
                Queries::GetTodo { user_id, todo_id } => {
                    Ok(query_as(r#"
                            select * from app_todo
                            where todo_id = $1 and user_id = $2
                        "#)
                        .bind(todo_id)
                        .bind(user_id)
                        .fetch_one(pool).await?)
                }
                Queries::DeleteTodo { todo_id, user_id } => {
                    Ok(query_as(r#"
                            delete from app_todo
                            where todo_id = $1 and user_id = $2
                            returning *
                        "#)
                        .bind(todo_id)
                        .bind(user_id)
                        .fetch_one(pool).await?)
                }
            }
        }
    }
}

pub async fn execute_transaction(
    database: &Database,
    transaction: Transactions,
) -> Result<(), AppErrors> {
    match database {
        Database::Postgres(pool) => match transaction {
            Transactions::CreateClientAndUser { user, client } => {
                let mut tx = pool.begin().await?;
                let _ = query_as(r#"
                            INSERT INTO app_user (user_id, email, created_at, updated_at, user_scopes, status, name, last_name)
                            VALUES ($1, $2, $3, $4, $5, $6,$7, $8)
                            RETURNING *
                        "#)
                    .bind(user.user_id)
                    .bind(user.email)
                    .bind(user.created_at)
                    .bind(user.updated_at)
                    .bind(user.user_scopes)
                    .bind(user.status)
                    .bind(user.name)
                    .bind(user.last_name)
                    .fetch_one(&mut *tx).await?;
                let _ = query_as(r#"
                            INSERT INTO app_client (client_id, user_id,client_description, client_secret, client_scopes, created_at, updated_at, expires_at, status)
                            VALUES ($1, $2, $3, $4, $5, $6,$7, $8, $9)
                            RETURNING *
                        "#)
                    .bind(client.client_id)
                    .bind(client.user_id)
                    .bind(client.client_description)
                    .bind(string_to_sha_256(client.client_secret))
                    .bind(client.client_scopes)
                    .bind(client.created_at)
                    .bind(client.updated_at)
                    .bind(client.expires_at)
                    .bind(client.status)
                    .fetch_one(&mut *tx).await?;
                tx.commit().await?;
                Ok(())
            }
        },
    }
}
