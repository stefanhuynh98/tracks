CREATE TABLE `users` (
	`pk` INT PRIMARY KEY AUTO_INCREMENT,
	`username` VARCHAR(255) NOT NULL,
	`oauth_id` INT NOT NULL,
	`oauth_provider` ENUM("github") NOT NULL, /* list will grow later on */
	`registered_since` DATETIME
);

ALTER TABLE `users` ADD CONSTRAINT uc_user unique (`username`, `oauth_provider`, `oauth_id`);
