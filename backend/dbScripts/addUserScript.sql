-- my local mysql configuration
CREATE USER 'diesel'@'localhost' IDENTIFIED BY 'password';
GRANT ALL PRIVILEGES ON chessDev.* TO 'diesel'@'localhost';
