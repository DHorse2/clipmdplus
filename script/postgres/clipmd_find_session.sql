-- Find sessions in the postgres:
select pid as process_id,
usename as username,
datname as database_name,
client_addr as client_address,
application_name,
backend_start,
state,
state_change,query
from pg_stat_activity;

 -- For specific database:
select pid as process_id,
usename as username,
datname as database_name,
client_addr as client_address,
application_name,
backend_start,
state,
state_change,query 
from pg_stat_activity where datname='dbaclass';

-- process_id | username | database_name | client_address | application_name |          backend_start           | state  |           state_change           ------------+----------+---------------+----------------+------------------+----------------------------------+--------+----------------------------------
--       18970 | postgres | dbaclass      |                | psql             | 2020-07-03 20:27:42.225987+05:30 | active | 2020-07-03 23:19:12.023416+05:30
-- (1 row)
