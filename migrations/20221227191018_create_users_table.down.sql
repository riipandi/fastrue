-- Add down migration script here
DROP INDEX public.users_instance_id_email_idx;
DROP INDEX public.users_instance_id_idx;
DROP TABLE IF EXISTS public.users;
