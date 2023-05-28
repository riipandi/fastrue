CREATE USER fastrue_auth_admin NOINHERIT CREATEROLE LOGIN NOREPLICATION PASSWORD 'securedbpass';
CREATE SCHEMA IF NOT EXISTS public AUTHORIZATION fastrue_auth_admin;
GRANT CREATE ON DATABASE postgres TO fastrue_auth_admin;
ALTER USER fastrue_auth_admin SET search_path = 'public';
