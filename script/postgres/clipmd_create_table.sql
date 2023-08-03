

-- -- Create a simple table
-- postgres=# Create table member_table ( mem_id integer, member_name varchar(100) , mobile integer not null);
-- CREATE TABLE

-- -- Create table with primary key
-- postgres=# Create table member_table ( mem_id integer primary key, member_name varchar(100) , mobile integer not null);
-- CREATE TABLE
-- -- Create table under particular tablespace
-- postgres=# Create table member_table ( mem_id integer primary key, member_name varchar(100) , mobile integer not null) tablespace pg_production_ts;
-- CREATE TABLE

-- -- Create table with unique constraint
-- postgres=# Create table member_table ( mem_id integer, member_name varchar(100) , mobile integer not null , constraint mem_id_cons unique(mem_id));
-- CREATE TABLE

-- -- Create temporary table:

-- postgres=# Create temporary table member_table ( mem_id integer primary key, member_name varchar(100) , mobile integer not null) tablespace pg_default;
-- CREATE TABLE
SET bytea_output = 'hex';
create table 
clip_data (
    id_key INTEGER PRIMARY KEY,
    id_system INTEGER       NOT NULL,
    data_creation_time DATE NOT NULL,
    data_processed BOOLEAN,
    data_synced DATE,
    sequence_number INTEGER,
    data_type VARCHAR(100),
    clip_data VARCHAR,
    clip_object BYTEA
)  tablespace SysMd;
