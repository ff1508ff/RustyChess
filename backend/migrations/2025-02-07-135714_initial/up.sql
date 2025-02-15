-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Games (
    id INT AUTO_INCREMENT PRIMARY KEY,
    state BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS StartPositions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    state BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS Pieces (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS Movements (
    id INT AUTO_INCREMENT PRIMARY KEY,
    rule BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS MovementPieces (
    id INT AUTO_INCREMENT PRIMARY KEY,
    movementsId INT NOT NULL,
    piescesId INT NOT NULL,
    FOREIGN KEY (movementsID) references Movements(id),
    FOREIGN KEY (piescesId) references Pieces(id)
)

