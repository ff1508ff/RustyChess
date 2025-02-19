-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Games (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    State BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS StartPositions (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    State BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS Pieces (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    Name VARCHAR(50) NOT NULL,
    Movement VARCHAR(255) NOT NULL
);
