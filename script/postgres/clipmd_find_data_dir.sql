

-- HOW TO FIND DATA DIRECTORY LOCATION 

-- DATA_DIRECTORY - > Specifies the directory to use for data storage.
show data_directory;

-- dbaclass=# show data_directory;

-- data_directory
-- -----------------------------
-- /Library/PostgreSQL/10/data
-- (1 row)

select setting from pg_settings where name = 'data_directory';
-- dbaclass=# select setting from pg_settings where name = 'data_directory';

-- setting
-- -----------------------------
-- /Library/PostgreSQL/10/data
-- (1 row)

-- This will show location of important files in postgres
SELECT name, setting FROM pg_settings WHERE category = 'File Locations';
-- dbaclass=# SELECT name, setting FROM pg_settings WHERE category = 'File Locations';
-- name                     | setting
-- -------------------+---------------------------------------------
-- config_file              | /Library/PostgreSQL/10/data/postgresql.conf
-- data_directory           | /Library/PostgreSQL/10/data
-- external_pid_file        |
-- hba_file                 | /Library/PostgreSQL/10/data/pg_hba.conf
-- ident_file               | /Library/PostgreSQL/10/data/pg_ident.conf
-- (5 rows)
