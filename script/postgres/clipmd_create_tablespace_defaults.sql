-- postgres=# 
-- show default_tablespace;
-- default_tablespace
-- --------------------
-- (1 row)

-- <<<< If output is blank means default is pg_default tablespace>>>>>

-- --To change the default tablespace at database level:
-- postgres=# 
alter system set default_tablespace=ClipMd;
-- ALTER SYSTEM
-- postgres=# 
-- select pg_reload_conf();
-- pg_reload_conf
-- ----------------
-- t
-- (1 row)

-- postgres=# 
-- show default_tablespace;
-- default_tablespace
-- --------------------
-- ts_postgres
-- (1 row)

-- postgres=# 
-- SELECT name, setting FROM pg_settings where name='default_tablespace';
-- name | setting
-- --------------------+-------------
-- default_tablespace | ts_postgres
-- (1 row)

-- Steps to change default tablespace at session level:
-- postgres=# 
set default_tablespace=ClipMd;
-- SET


-- -----------------------------------------------------------------------------------


-- VIEW DEFAULT TEMP TABLESPACE:
-- dbaclass=# 
SELECT name, setting FROM pg_settings where name='temp_tablespaces';
-- name | setting
-- ------------------+---------
-- temp_tablespaces |
-- (1 row)

-- dbaclass=# 
show temp_tablespaces;
-- dbaclass-# ;
-- temp_tablespaces
-- ------------------
-- (1 row)

-- CHANGE DEFAULT TEMP TABLESPACE
-- 
-- postgres=# 
alter system set temp_tablespaces=ClipMd;
-- ALTER SYSTEM

-- postgres=# 
select pg_reload_conf();
-- pg_reload_conf
-- ----------------
-- t
-- (1 row)

-- postgres=# 
show temp_tablespaces;
-- temp_tablespaces
-- ------------------
-- ts_temp
-- (1 row)

-- postgres=# 
SELECT name, setting FROM pg_settings where name='temp_tablespaces';
-- name | setting
-- ------------------+---------
-- temp_tablespaces | ts_temp
-- (1 row)
