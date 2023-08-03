-- How to get postgres db size:
SELECT pg_database.datname as "database_name", pg_size_pretty(pg_database_size(pg_database.datname)) AS size_in_mb FROM pg_database ORDER by size_in_mb DESC;
-- postgres=# SELECT pg_database.datname as "database_name", pg_size_pretty(pg_database_size(pg_database.datname)) AS size_in_mb FROM pg_database ORDER by size_in_mb DESC;

--  database_name | size_in_mb
-- ---------------+------------
--  DBACLASS      | 7767 kB
--  postgres      | 7735 kB
--  template1     | 7735 kB
--  template0     | 7601 kB
-- (4 rows)
-- (or)
-- postgres=# \l+
--                                                                List of databases
--    Name    |  Owner   | Encoding | Collate | Ctype |   Access privileges   |  Size   | Tablespace |                Description
-- -----------+----------+----------+---------+-------+-----------------------+---------+------------+--------------------------------------------
--  DBACLASS  | postgres | UTF8     | C       | C     |                       | 7767 kB | pg_default | TESTING DB
--  postgres  | postgres | UTF8     | C       | C     |                       | 7735 kB | pg_default | default administrative connection database
--  template0 | postgres | UTF8     | C       | C     | =c/postgres          +| 7601 kB | pg_default | unmodifiable empty database
--            |          |          |         |       | postgres=CTc/postgres |         |            |
--  template1 | postgres | UTF8     | C       | C     | =c/postgres          +| 7735 kB | pg_default | default template for new databases
--            |          |          |         |       | postgres=CTc/postgres |         |            |
-- (4 rows)
