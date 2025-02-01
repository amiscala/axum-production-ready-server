CREATE TABLE app_client(
   todo_id UUID PRIMARY KEY,
   user_id UUID NOT NULL,
   FOREIGN KEY (user_id) REFERENCES app_user (user_id) ON DELETE CASCADE,
   title TEXT NOT NULL,
   body TEXT NOT NULL,
   category TEXT NOT NULL,
   created_at timestamp with time zone NOT NULL,
   updated_at timestamp with time zone NOT NULL,
   status TEXT NOT NULL
);