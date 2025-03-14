CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    passwordhash VARCHAR(256) NOT NULL,
    admin BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS posts (
    id SERIAL PRIMARY KEY,
    author INTEGER REFERENCES users(id) ON DELETE SET NULL,
    title VARCHAR(256) NOT NULL,
    markdown_content TEXT NOT NULL,
    slug VARCHAR(256) NOT NULL,
    description TEXT NOT NULL,
    released BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

-- for videos, code snippets, videos or gifs
CREATE TABLE IF NOT EXISTS media (
    id SERIAL PRIMARY KEY,
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    data BYTEA NOT NULL,
    media_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
