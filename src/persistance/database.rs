use std::sync::Arc;
use sqlx::{Postgres, Pool, query_as, FromRow};
use crate::domain::{AppErrors, Queries};
use crate::domain::models::user::User;

#[derive(Clone)]
pub enum Database {
    Postgres(Pool<Postgres>)
}

pub async fn execute_query<T>(database: &Database, query: Queries) -> Result<T, AppErrors>
where
    T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    match database {
        Database::Postgres(pool) => {
            match query {
                Queries::CreateUser { user } => {
                    Ok(sqlx::query_as::<_, T>(r#"
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
                        .fetch_one(&*pool).await?)
                }
                Queries::UpdateUser { user_id, email, name, last_name, updated_at } => {
                    Ok(query_as(r#"
                            UPDATE app_user
                            set email = $1, name=$2, last_name=$3, updated_at = $4
                            where user_id=$5
                        "#)
                        .bind(email)
                        .bind(name)
                        .bind(last_name)
                        .bind(updated_at)
                        .bind(user_id)
                        .fetch_one(&*pool).await?)
                }
                Queries::DeleteUser { user_id } => {
                    Ok(query_as(r#"
                            delete from app_user
                            where user_id = $1
                            returning *
                        "#)
                        .bind(user_id).fetch_one(&*pool).await?)
                }

                Queries::GetUser { user_id } => {
                    Ok(query_as(r#"
                            select * from app_user
                            where user_id = $1
                        "#)
                        .bind(user_id)
                        .fetch_one(&*pool).await?)
                }
                Queries::CreateClientAndUser { .. } => {
                    // let mut tx = &*pool.begin().await?;
                    // let user_result = query(r#"
                    //
                    // "#)
                    todo!()
                }
                Queries::CreateClient { .. } => {
                    todo!()
                }
                Queries::UpdateClient { .. } => {
                    todo!()
                }
                Queries::DeleteClient { .. } => {
                    todo!()
                }
                Queries::GetClient { .. } => {
                    todo!()
                }
                Queries::GetClients { .. } => {
                    todo!()
                }
            }
        }
    }
}