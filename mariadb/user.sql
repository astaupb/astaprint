--  AStAPrint - Database - User Tables
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

CREATE TABLE `user`(
  `id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR(32) UNIQUE NOT NULL,
  `hash` BINARY(64) NOT NULL,
  `salt` BINARY(16) NOT NULL,
  `card` BIGINT UNSIGNED UNIQUE,,
  `pin` INT UNSIGNED,
  `locked`  BOOLEAN NOT NULL DEFAULT false,
  `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP);

CREATE TABLE `user_tokens`(
  `id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
  `user_id` INT UNSIGNED NOT NULL,
  `user_agent` VARCHAR(128) NOT NULL,
  `ip` VARCHAR(48) NOT NULL,
  `location` VARCHAR(64) NOT NULL,
  `hash` BINARY(64) NOT NULL,
  `created` TIMESTAMP NOT NULL);
