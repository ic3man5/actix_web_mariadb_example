CREATE TABLE IF NOT EXISTS users(
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(128) NOT NULL,
    -- RFC 3696: An email address must not exceed 254 characters. */
    passwd BYTEA NOT NULL,
    email VARCHAR(256) NOT NULL
);

INSERT INTO users(username, passwd, email)
VALUES ('Dave', sha256('changeme'), 'dave@gmail.com'),
       ('Bill', sha256('password1'), 'bill@gmail.com');