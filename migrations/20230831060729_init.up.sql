CREATE TABLE contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Populate the table with some demo data

INSERT INTO contacts (full_name, phone, email)
VALUES ('John Doe', '555-111-1111', 'john.doe@example.com');

INSERT INTO contacts (full_name, phone, email)
VALUES ('Jane Doe', '555-222-2222', 'jane.doe@example.com');
