
--  \list+
-- postgres=# \list+
-- List of databases
-- Name | Owner | Encoding | Collate | Ctype | Access privileges | Size | Tablespace | Description
-- -----------+----------+----------+---------+-------+-----------------------+---------+------------+--------------------------------------------
-- dbaclass | postgres | UTF8 | C | C | | 2268 MB | pg_default |
-- postgres | postgres | UTF8 | C | C | | 4132 MB | pg_default | default administrative connection database
-- template0 | postgres | UTF8 | C | C | =c/postgres +| 7601 kB | pg_default | unmodifiable empty database
-- | | | | | postgres=CTc/postgres | | |
-- template1 | postgres | UTF8 | C | C | =c/postgres +| 7601 kB | pg_default | default template for new databases
-- | | | | | postgres=CTc/postgres | | |
-- (4 rows)

select datname from pg_database;
-- postgres=# select datname from pg_database;
-- datname
-- -----------
-- postgres
-- template1
-- template0
-- dbaclass
-- (4 rows)
