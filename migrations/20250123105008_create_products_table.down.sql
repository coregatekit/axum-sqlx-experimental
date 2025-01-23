-- Add down migration script here
CREATE TABLE
    IF NOT EXISTS products (
        id CHAR(36) PRIMARY KEY NOT NULL,
        title VARCHAR(255) NOT NULL,
        img VARCHAR(255) NOT NULL,
        price DECIMAL(10, 2) NOT NULL,
        is_active BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );