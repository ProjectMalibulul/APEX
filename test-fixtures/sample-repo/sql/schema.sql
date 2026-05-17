CREATE TABLE users (
  id TEXT PRIMARY KEY
);

CREATE TABLE posts (
  id TEXT PRIMARY KEY,
  user_id TEXT REFERENCES users(id)
);

