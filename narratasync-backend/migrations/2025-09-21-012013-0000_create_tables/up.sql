-- Create users table
CREATE TABLE users (
    id VARCHAR(80) PRIMARY KEY,
    username TEXT NOT NULL,
    display_name TEXT,
    email TEXT,
    token TEXT NOT NULL
);

-- Create scenario table
CREATE TABLE scenario (
    id VARCHAR(80) PRIMARY KEY,
    title TEXT NOT NULL,
    thumbnail TEXT,
    author VARCHAR(80),
    FOREIGN KEY (author) REFERENCES users (id)
);

-- Insert some random stuff into the database
INSERT INTO users (id, username, display_name, email, token) VALUES
('user1', 'alice', 'Alice Wonderland', 'alice@example.com', 'token1'),
('user2', 'bob', 'Bob Builder', 'bob@example.com', 'token2');

INSERT INTO scenario (id, title, thumbnail, author) VALUES
('scenario1', 'The Great Adventure', 'https://picsum.photos/200/300?random=1', 'user1'),
('scenario2', 'Mystery of the Old House', 'https://picsum.photos/200/300?random=2', 'user2'),
('scenario3', 'Journey to the Unknown', 'https://picsum.photos/200/300?random=3', 'user1');