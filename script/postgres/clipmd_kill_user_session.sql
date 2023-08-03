-- Here we want to kill all session of the user postgres

-- List all the session of that user.
dbaclass#select datname as database, pid as pid, usename as username, application_name , client_addr, query FROM pg_stat_activity where username='postgres';

-- Kill all the session of user postgres.

dbaclass#select pg_terminate_backend(pid) from pg_stat_activity where usename='postgres';
