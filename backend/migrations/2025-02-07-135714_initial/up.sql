-- Your SQL goes here

CREATE TABLE IF NOT EXISTS Role
(
    Id   INT AUTO_INCREMENT PRIMARY KEY,
    Name VARCHAR(255) NOT NULL
);

INSERT INTO Role (Id, Name)
VALUES (1, 'Admin'),
       (2, 'Moderator'),
       (3, 'Player');

CREATE TABLE IF NOT EXISTS User
(
    Id        INT AUTO_INCREMENT PRIMARY KEY,
    RoleId    INT          NOT NULL,
    Username  VARCHAR(100) NOT NULL,
    Email     VARCHAR(255) NOT NULL,
    CreatedAt DATETIME     NOT NULL,

    FOREIGN KEY (RoleId) REFERENCES `Role` (Id)
);

CREATE TABLE IF NOT EXISTS Game
(
    Id            INT AUTO_INCREMENT PRIMARY KEY,
    State         BLOB     NOT NULL,
    PlayerWhiteId INT      NOT NULL,
    PlayerBlackId INT      NOT NULL,
    CreatedAt     DATETIME NOT NULL,
    EndedAt       DATETIME,

    FOREIGN KEY (PlayerWhiteId) REFERENCES `User` (Id),
    FOREIGN KEY (PlayerBlackId) REFERENCES `User` (Id)
);

CREATE TABLE IF NOT EXISTS StartPosition
(
    Id    INT AUTO_INCREMENT PRIMARY KEY,
    State BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS Pawn
(
    Id       INT AUTO_INCREMENT PRIMARY KEY,
    Name     VARCHAR(50)  NOT NULL,
    Movement VARCHAR(255) NOT NULL
);