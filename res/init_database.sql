-- This script contains all the initialization procedures for the Player  
-- Collection database
--
-- To appy changes to an existing database, just run this script once again, it 
-- will execute all the ALTER procedures it contains without whipping off the 
-- existing data.
--
-- Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
 
 -- Creates the Player collection database and its tables
 -- # Tables:
 -- * character_unique
CREATE DATABASE playercollectiondb;

-- Go to player collection
\c playercollectiondb

CREATE TABLE IF NOT EXISTS character (
	id BIGSERIAL PRIMARY KEY,
	player VARCHAR(64) NOT NULL,
	character BIGINT NOT NULL
);
