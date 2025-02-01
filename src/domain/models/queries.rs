use crate::domain::models::client::Client;
use crate::domain::models::todo::Todo;
use crate::domain::models::user::User;
use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

pub enum Queries {
    CreateUser {
        user: User,
    },
    UpdateUser {
        user_id: Uuid,
        email: String,
        name: String,
        last_name: String,
    },
    DeleteUser {
        user_id: Uuid,
    },
    GetUser {
        user_id: Uuid,
    },

    CreateClient {
        client: Client,
    },
    UpdateClient {
        user_id: Uuid,
        client_id: Uuid,
        client_description: String,
        client_scopes: String,
        expires_at: Option<DateTime<Utc>>,
    },
    DeleteClient {
        user_id: Uuid,
        client_id: Uuid,
    },
    GetClient {
        user_id: Uuid,
        client_id: Uuid,
    },
    GetClientWithClientIdAndClientSecret {
        client_id: Uuid,
        client_secret: String,
    },
    CreateTodo {
        todo: Todo,
    },
    UpdateTodo {
        user_id: Uuid,
        todo_id: Uuid,
        title: String,
        body: String,
        category: String,
        status: String,
    },
    DeleteTodo {
        user_id: Uuid,
        todo_id: Uuid,
    },
    GetTodo {
        user_id: Uuid,
        todo_id: Uuid,
    },
}

pub enum VecQueries {
    GetTodos { user_id: Uuid },
    GetClients { user_id: Uuid },
}

pub enum Transactions {
    CreateClientAndUser { user: User, client: Client },
}
