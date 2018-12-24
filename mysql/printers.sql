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
	`counter_id` INT UNSIGNED NOT NULL,
	`control_id` INT UNSIGNED NOT NULL,
	`status_id` INT UNSIGNED NOT NULL,
	`info_id` INT UNSIGNED NOT NULL,
	`location` VARCHAR(42) NOT NULL,
    `has_a3` BOOLEAN NOT NULL,
    `coin_operated` BOOLEAN NOT NULL,
	`description` VARCHAR(32) NOT NULL,
    `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP);

CREATE TABLE `printer_counter`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
	`total` VARCHAR(128) NOT NULL,
	`copy_total` VARCHAR(128) NOT NULL,
	`copy_bw` VARCHAR(128) NOT NULL,
	`print_total` VARCHAR(128) NOT NULL,
	`print_bw` VARCHAR(128) NOT NULL,
    `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE `printer_control`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `queue` VARCHAR(128) NOT NULL,
	`cancel` INT NOT NULL,
	`clear` INT NOT NULL,
    `energy` VARCHAR(128) NOT NULL,
    `wake` INT NOT NULL,
    `sleep` INT NOT NULL,
    `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE `printer_info`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `model` VARCHAR(128) NOT NULL,
    `hostname` VARCHAR(128) NOT NULL,
    `location` VARCHAR(128) NOT NULL,
    `mac` VARCHAR(128) NOT NULL,
    `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE `printer_status`(
	`id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT NOT NULL,
    `uptime` VARCHAR(128) NOT NULL,
    `scan` VARCHAR(128) NOT NULL,
    `copy` VARCHAR(128) NOT NULL,
    `toner` VARCHAR(128) NOT NULL,
    `tray_1` VARCHAR(128) NOT NULL,
    `tray_2` VARCHAR(128) NOT NULL,
    `tray_3` VARCHAR(128) NOT NULL,
    `tray_4` VARCHAR(128) NOT NULL,
    `tray_5` VARCHAR(128) NOT NULL,
    `created` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
