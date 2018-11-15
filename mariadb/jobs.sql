--  AStAPrint - Database - Jobs Tables
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

CREATE TABLE `jobs`(
  `id` INT UNSIGNED UNIQUE PRIMARY KEY AUTO_INCREMENT,
  `user_id` INT UNSIGNED,
  `info` BINARY(128) NOT NULL,
  `options` BINARY(182) NOT NULL,
  `data` LONGBLOB NOT NULL,
  `preview_0` MEDIUMBLOB NOT NULL,
  `preview_1` MEDIUMBLOB,
  `preview_2` MEDIUMBLOB,
  `preview_3` MEDIUMBLOB,
  `created` TIMESTAMP NOT NULL,
  `updated` TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP);

