

-- -- Find the sequence details:

-- select * from pg_sequences;

-- (or)

-- \ds+

-- List of relations
-- Schema  | Name      | Type     | Owner        | Size | Description
-- --------+-----------+----------+--------------+------------+-------------
-- public  | class_seq | sequence | enterprisedb | 8192 bytes |
-- (1 row)

-- -- Create sequences:

-- postgres# CREATE SEQUENCE class_seq INCREMENT 1 MINVALUE 1 MAXVALUE 1000 START 1;
-- CREATE SEQUENCE

-- -- Create sequence in descending:

-- postgres# CREATE SEQUENCE class_seq INCREMENT -1 MINVALUE 1 MAXVALUE 1000 START 1000;
-- CREATE SEQUENCE

-- -- Alter sequence to change maxvalue:

-- postgres=# alter sequence class_seq maxvalue 500;
-- ALTER SEQUENCE

-- -- Reset a sequence using alter command:

-- postgres=# alter sequence class_seq restart with 1;
-- ALTER SEQUENCE

-- -- Find next_val and currval of a sequence:

-- postgres=# select nextval('class_seq');
-- nextval
-- ---------
-- 1
-- (1 row)

-- postgres=# select currval('class_seq');
-- currval
-- ---------
-- 1
-- (1 row)
