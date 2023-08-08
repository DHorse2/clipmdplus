--  todo Analisis and create index creation script

-- -- Simple create index:
-- postgres=# create index tab_idx2 on scott.customer(emp_name);
-- CREATE INDEX

-- -- Create index with tablespace:

-- postgres#CREATE INDEX tab_idx2 on scott.customer(emp_name) TABLESPACE IND_TS;
-- CREATE INDEX

-- -- Create index without causing blocking:

-- postgres=# create index concurrently tab_idx2 on scott.customer(emp_name) TABLESPACE IND_TS;
-- CREATE INDEX

-- -- Create unique index:

-- postgres=# create unique index tab_idx2 on scott.customer(emp_name)
-- CREATE INDEX

-- -- Create functional index:

-- postgres=# create index fun_idx on scott.customer(lower(emp_name));
-- CREATE INDEX

-- -- Create multi column index:

-- postgres=# create index multi_idx on scott.customer(emp_name,emp_id);
-- CREATE INDEX

