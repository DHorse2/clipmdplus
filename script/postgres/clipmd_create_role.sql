-- ! todo
CREATE ROLE Clipboard

-- CREATE ROLE name [ [ WITH ] option [ ... ] ]

-- where option can be:

--       SUPERUSER | NOSUPERUSER
--     | CREATEDB | NOCREATEDB
--     | CREATEROLE | NOCREATEROLE
--     | INHERIT | NOINHERIT
--     | LOGIN | NOLOGIN
--     | REPLICATION | NOREPLICATION
--     | BYPASSRLS | NOBYPASSRLS
--     | CONNECTION LIMIT connlimit
--     | [ ENCRYPTED ] PASSWORD 'password' | PASSWORD NULL
--     | VALID UNTIL 'timestamp'
--     | IN ROLE role_name [, ...]
--     | IN GROUP role_name [, ...]
--     | ROLE role_name [, ...]
--     | ADMIN role_name [, ...]
--     | USER role_name [, ...]
--     | SYSID uid