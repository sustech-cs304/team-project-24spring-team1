CREATE FUNCTION head(text) RETURNS text AS $$
BEGIN
    RETURN left($1, 256);
END;
$$ LANGUAGE plpgsql;

CREATE TYPE Role AS ENUM ('admin', 'staff', 'student');
CREATE TYPE EventType AS ENUM ('show', 'lecture', 'competition', 'other');

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    sustech_id INT NOT NULL UNIQUE CHECK (1000000 <= sustech_id AND sustech_id <= 99999999),
    name VARCHAR(30) NOT NULL UNIQUE,
    email VARCHAR(48) NOT NULL UNIQUE,
    password VARCHAR(128),
    avatar UUID NOT NULL DEFAULT '98e90a01-9fd6-4010-add4-9190e8ee0b6c',  -- magic value
    role Role NOT NULL DEFAULT 'student',
    bio TEXT NOT NULL DEFAULT '',
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE places (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(30) NOT NULL UNIQUE
);

CREATE TABLE events (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    kind EventType NOT NULL,
    description TEXT NOT NULL,
    cover UUID NOT NULL DEFAULT '80e8f45c-6fbc-4f94-909d-d58462e845d5',  -- magic value
    organizer_id INT NOT NULL REFERENCES accounts(id),
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    venue_id INT NOT NULL REFERENCES places(id),
    location POINT NOT NULL,
    tickets INT,
    registration_deadline TIMESTAMP,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE participation (
    account_id INT NOT NULL REFERENCES accounts(id),
    event_id INT NOT NULL REFERENCES events(id),
    PRIMARY KEY (account_id, event_id)
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INT NOT NULL REFERENCES accounts(id),
    event_id INT NOT NULL REFERENCES events(id),
    content TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE moments (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INT NOT NULL REFERENCES accounts(id),
    content TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE moment_comments (
    id SERIAL PRIMARY KEY NOT NULL,
    account_id INT NOT NULL REFERENCES accounts(id),
    moment_id INT NOT NULL REFERENCES moments(id),
    content TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE chats (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(30) NOT NULL,
    is_group BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_members (
    chat_id INT NOT NULL REFERENCES chats(id),
    account_id INT NOT NULL REFERENCES accounts(id),
    last_read TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, account_id)
);

CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY NOT NULL,
    chat_id INT NOT NULL REFERENCES chats(id),
    account_id INT NOT NULL REFERENCES accounts(id),
    content TEXT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX ON events (kind);
CREATE INDEX ON events (start_at);
CREATE INDEX ON events (end_at);
CREATE INDEX ON events (COALESCE(registration_deadline, end_at));

SELECT diesel_manage_updated_at('accounts');
SELECT diesel_manage_updated_at('events');
