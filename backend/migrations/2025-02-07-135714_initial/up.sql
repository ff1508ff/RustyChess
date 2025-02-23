-- Your SQL goes here

CREATE TABLE roles
(
    id   INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

INSERT INTO roles (id, name)
VALUES (1, 'Admin'),
       (2, 'Moderator'),
       (3, 'Player');

CREATE TABLE users
(
    id         INT AUTO_INCREMENT PRIMARY KEY,
    role_id    INT          NOT NULL,
    username   VARCHAR(100) NOT NULL,
    email      VARCHAR(255) NOT NULL,
    created_at DATETIME     NOT NULL,

    FOREIGN KEY (role_id) REFERENCES `roles` (id)
);

CREATE TABLE games
(
    id              INT AUTO_INCREMENT PRIMARY KEY,
    state           BLOB     NOT NULL,
    player_id_white INT      NOT NULL,
    player_id_black INT      NOT NULL,
    created_at      DATETIME NOT NULL,
    ended_at        DATETIME,

    FOREIGN KEY (player_id_white) REFERENCES `users` (id),
    FOREIGN KEY (player_id_black) REFERENCES `users` (id)
);

CREATE TABLE startPositions
(
    id    INT AUTO_INCREMENT PRIMARY KEY,
    state BLOB NOT NULL
);

CREATE TABLE pawns
(
    id       INT AUTO_INCREMENT PRIMARY KEY,
    name     VARCHAR(50)  NOT NULL,
    movement VARCHAR(255) NOT NULL
);
