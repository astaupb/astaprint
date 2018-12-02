--  AStAPrint - Database - Journal Tables
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
  `description` VARCHAR(128) NOT NULL,
  `created` TIMESTAMP NOT NULL);

CREATE TABLE `journal_digest`(
  `id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
  `digest` BINARY(64) NOT NULL,
  `credit` DECIMAL(7, 2) NOT NULL,
  `created` TIMESTAMP NOT NULL);

CREATE TABLE `journal_tokens`(
  `id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
  `value` DECIMAL(5, 2) NOT NULL DEFAULT 1.0,
  `content` VARCHAR(128) NOT NULL,
  `used` BOOLEAN NOT NULL,
  `used_by` INT UNSIGNED,
  `created` TIMESTAMP NOT NULL,
  `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP);

-- seed journal_digest
INSERT INTO `journal_digest`(digest, credit) VALUES (UNHEX(SHA2(NOW(), 512)), 0.0);

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
