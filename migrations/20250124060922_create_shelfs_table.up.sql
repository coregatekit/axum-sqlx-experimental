-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS shelfs (
        id SERIAL PRIMARY KEY,
        product_id INT NOT NULL,
        quantity INT NOT NULL,
        is_active BOOLEAN DEFAULT TRUE,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (product_id) REFERENCES product (id) ON DELETE CASCADE
    );