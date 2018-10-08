--  AStAPrint-Database - User Create
--  Copyright (C) 2018  AStA der Universität Paderborn
-- 
--  Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
-- 
--  This program is free software: you can redistribute it and/or modify
--  it under the terms of the GNU Affero General Public License as published by
--  the Free Software Foundation, either version 3 of the License, or
--  (at your option) any later version.
-- 
--  This program is distributed in the hope that it will be useful,
--  but WITHOUT ANY WARRANTY; without even the implied warranty of
--  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
--  GNU Affero General Public License for more details.
-- 
--  You should have received a copy of the GNU Affero General Public License
--  along with this program.  If not, see <https://www.gnu.org/licenses/>.

CREATE TABLE `journal`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
	`user_id` INT UNSIGNED NOT NULL,
	`value` DECIMAL(7, 2) NOT NULL,
	`credit` DECIMAL(7, 2) NOT NULL,
	`description` VARCHAR(128) NOT NULL,
	`timestamp` TIMESTAMP NOT NULL);

CREATE TABLE `journal_digest`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
	`digest` BINARY(64) NOT NULL,
	`timestamp` TIMESTAMP NOT NULL);
	
-- seed journal digests
INSERT INTO `journal_digest`(digest) VALUES (UNHEX(SHA2(NOW(), 512)));

CREATE TABLE `user`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
	`name` VARCHAR(32) UNIQUE NOT NULL,
	`locked`  BOOLEAN NOT NULL,
	`pin_hash` BINARY(64) DEFAULT NULL,
	`pin_salt` BINARY(16) DEFAULT NULL,
	`password_hash` BINARY(64) NOT NULL,
	`password_salt` BINARY(16) NOT NULL,
	`timestamp` TIMESTAMP NOT NULL);

CREATE TABLE `token`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
	`user_id` INT UNSIGNED NOT NULL,
	`user_agent` VARCHAR(64) NOT NULL,
	`location` VARCHAR(64) NOT NULL,
	`value` BINARY(64) NOT NULL,
	`timestamp` TIMESTAMP NOT NULL);

CREATE TABLE `register_token`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
	`value` VARCHAR(16) UNIQUE NOT NULL,
	`used` BOOLEAN NOT NULL DEFAULT FALSE,
	`user_id` INT UNSIGNED DEFAULT NULL,
	`timestamp` TIMESTAMP NOT NULL);

DELIMITER $$
CREATE TRIGGER `journal_insert`
AFTER INSERT ON `journal`
FOR EACH ROW BEGIN

SET @current_id := (SELECT `auto_increment` 
						FROM INFORMATION_SCHEMA.TABLES
						WHERE table_name = "journal_digest") - 1;

SET @digest := UNHEX((SELECT 
					SHA2(CONCAT(
						jd.digest, j.id, j.user_id, j.value, j.credit, j.description, j.timestamp), 512)
				FROM journal j
				INNER JOIN journal_digest jd
				ON j.id = jd.id
				WHERE j.id = @current_id));

INSERT INTO journal_digest(digest) VALUES (@digest);
				
END $$
DELIMITER ;

CREATE TRIGGER `journal_update`
BEFORE UPDATE ON `journal`
FOR EACH ROW SIGNAL SQLSTATE "45000"
	SET MESSAGE_TEXT = "update on journal not allowed";

CREATE TRIGGER `journal_delete`
BEFORE DELETE ON `journal`
FOR EACH ROW SIGNAL SQLSTATE "45000"
	SET MESSAGE_TEXT = "delete on journal not allowed";

CREATE TRIGGER `journal_digest_update`
BEFORE UPDATE ON `journal_digest`
FOR EACH ROW SIGNAL SQLSTATE "45000"
	SET MESSAGE_TEXT = "update on journal_digest not allowed";

CREATE TRIGGER `journal_digest_delete`
BEFORE DELETE ON `journal_digest`
FOR EACH ROW SIGNAL SQLSTATE "45000"
	SET MESSAGE_TEXT = "delete on journal_digest not allowed";
