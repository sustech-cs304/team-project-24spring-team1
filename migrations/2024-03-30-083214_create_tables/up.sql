CREATE FUNCTION head(text) RETURNS text AS $$
BEGIN
    RETURN left($1, 256);
END;
$$ LANGUAGE plpgsql;

CREATE TYPE Role AS ENUM ('admin', 'staff', 'student');
CREATE TYPE EventType AS ENUM ('show', 'lecture', 'competition', 'other');

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    sustech_id INT NOT NULL CHECK (1000000 <= sustech_id AND sustech_id <= 99999999),
    name VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL,
    role Role NOT NULL DEFAULT 'student',
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
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    venue_id INT NOT NULL REFERENCES places(id),
    description TEXT NOT NULL,
    organizer_id INT NOT NULL REFERENCES accounts(id),
    tickets INT,
    registeration_deadline TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE participation (
    account INT NOT NULL REFERENCES accounts(id),
    event INT NOT NULL REFERENCES events(id),
    PRIMARY KEY (account, event)
);

SELECT diesel_manage_updated_at('accounts');
SELECT diesel_manage_updated_at('events');
