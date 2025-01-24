-- Add migration script here
CREATE TABLE app_user(
     user_id UUID PRIMARY KEY,
     email TEXT NOT NULL UNIQUE,
     name TEXT NOT NULL,
     last_name TEXT NOT NULL,
     created_at timestamp with time zone NOT NULL,
     updated_at timestamp with time zone NOT NULL,
     user_scopes TEXT NOT NULL,
     status TEXT NOT NULL
);

CREATE TABLE app_client(
    client_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES app_user,
    FOREIGN KEY (user_id) REFERENCES app_user (user_id) ON DELETE CASCADE,
    client_description TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    client_scopes TEXT NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    expires_at timestamp with time zone,
    status TEXT NOT NULL
);