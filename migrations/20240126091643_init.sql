-- Add migration script here
CREATE TABLE IF NOT EXISTS `prices` (
    `id`              INTEGER PRIMARY KEY,
    `date`            DATE,
    `price`           REAL,
    `instrument_name` TEXT
);

-- CREATE TABLE `users` {
--   `id` int(11) NOT NULL AUTO_INCREMENT,
--   `name` varchar(255) NOT NULL,
--   `email` varchar(255) NOT NULL,
--   `password` varchar(255) NOT NULL,
--   PRIMARY KEY (`id`)
-- }
