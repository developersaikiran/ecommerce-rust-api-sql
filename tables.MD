CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  store_id INT,
  name VARCHAR(255) NOT NULL,
  price DECIMAL(10,2) NOT NULL,
  discount DECIMAL(5,2) DEFAULT 0,
  image TEXT,
  description TEXT,
  ingredients TEXT,
  how_to_use TEXT,
  quantity INT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);