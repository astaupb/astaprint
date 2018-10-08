--  AStAPrint-Database - Printer Create
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

CREATE TABLE `printer`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`hostname` VARCHAR(32) NOT NULL,
	`ip` VARCHAR(42) UNIQUE NOT NULL,
	`community` VARCHAR(42) NOT NULL,
	`mac` VARCHAR(18) NOT NULL,
	`device_id` SMALLINT UNSIGNED NOT NULL,
	`model_id` SMALLINT UNSIGNED NOT NULL,
	`location` VARCHAR(42) NOT NULL,
	`description` VARCHAR(32) NOT NULL
);

CREATE TABLE `model`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`counter_id` SMALLINT UNSIGNED NOT NULL,
	`queue_ctl_id` SMALLINT UNSIGNED NOT NULL,
	`energy_ctl_id` SMALLINT UNSIGNED NOT NULL,
	`name` VARCHAR(42) NOT NULL
);

CREATE TABLE `counter`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`total` VARCHAR(128) NOT NULL,
	`print_black` VARCHAR(128) NOT NULL,
	`print_color` VARCHAR(128),
	`copy_black` VARCHAR(128) NOT NULL,
	`copy_color` VARCHAR(128),
	`description` VARCHAR(42) NOT NULL
);

CREATE TABLE `queue_ctl`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`oid` VARCHAR(128) NOT NULL,
	`cancel` INT NOT NULL,
	`clear` INT NOT NULL
);

CREATE TABLE `energy_ctl`(
	`id` SMALLINT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`oid` VARCHAR(128) NOT NULL,
	`wake` INT NOT NULL,
	`sleep` INT NOT NULL
);
