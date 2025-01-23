CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    google_id VARCHAR UNIQUE NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP NOT NULL
);

INSERT INTO users (google_id, email, name, last_login)
VALUES ('example_google_id', 'me@example.com', 'Default', CURRENT_TIMESTAMP);
