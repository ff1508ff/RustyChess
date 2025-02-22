-- Your SQL goes here

CREATE TABLE IF NOT EXISTS role
(
    Id   INT AUTO_INCREMENT PRIMARY KEY,
    Name VARCHAR(255) NOT NULL
);

INSERT INTO role (Id, Name)
VALUES (1, 'Admin'),
       (2, 'Moderator'),
       (3, 'Player');

CREATE TABLE IF NOT EXISTS user
(
    Id        INT AUTO_INCREMENT PRIMARY KEY,
    RoleId    INT          NOT NULL,
    Username  VARCHAR(100) NOT NULL,
    Email     VARCHAR(255) NOT NULL,
    CreatedAt DATETIME     NOT NULL,

    FOREIGN KEY (RoleId) REFERENCES `role` (Id)
);

CREATE TABLE IF NOT EXISTS game
(
    Id            INT AUTO_INCREMENT PRIMARY KEY,
    State         BLOB     NOT NULL,
    PlayerWhiteId INT      NOT NULL,
    PlayerBlackId INT      NOT NULL,
    CreatedAt     DATETIME NOT NULL,
    EndedAt       DATETIME,

    FOREIGN KEY (PlayerWhiteId) REFERENCES `user` (Id),
    FOREIGN KEY (PlayerBlackId) REFERENCES `user` (Id)
);

CREATE TABLE IF NOT EXISTS startPosition
(
    Id    INT AUTO_INCREMENT PRIMARY KEY,
    State BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS pawn
(
    Id       INT AUTO_INCREMENT PRIMARY KEY,
    Name     VARCHAR(50)  NOT NULL,
    Movement VARCHAR(255) NOT NULL
);
