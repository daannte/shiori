CREATE UNIQUE INDEX users_username_unique_lower
ON users (LOWER(username))
