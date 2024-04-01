CREATE TYPE Role AS ENUM ('admin', 'staff', 'student');

CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    sustech_id INT NOT NULL CHECK (1000000 <= sustech_id AND sustech_id <= 99999999),
    name VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL,
    role Role NOT NULL DEFAULT 'student',
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('accounts');
