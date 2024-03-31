CREATE TABLE accounts (
    id SERIAL PRIMARY KEY NOT NULL,
    student_id INT NOT NULL CHECK (1000000 <= student_id AND student_id <= 99999999),
    name VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(128) NOT NULL,
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('accounts');
