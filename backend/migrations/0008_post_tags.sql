CREATE TABLE post_categories (
    id SERIAL PRIMARY KEY,
    category VARCHAR(256) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE post_tags (
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    category_id INTEGER REFERENCES post_categories(id) ON DELETE CASCADE,
    PRIMARY KEY(post_id, category_id)
);
