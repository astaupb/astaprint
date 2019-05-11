--  AStAPrint - Database - Printers Table
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

CREATE TABLE `printers`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`hostname` VARCHAR(32) NOT NULL,
	`ip` VARCHAR(42) UNIQUE NOT NULL,
	`community` VARCHAR(42) NOT NULL,
	`mac` VARCHAR(18) NOT NULL,
	`device_id` INT UNSIGNED NOT NULL,
	`location` VARCHAR(42) NOT NULL,
	`has_a3` BOOLEAN NOT NULL,
	`coin_operated` BOOLEAN NOT NULL,
	`description` VARCHAR(32) NOT NULL,
	`created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP);
