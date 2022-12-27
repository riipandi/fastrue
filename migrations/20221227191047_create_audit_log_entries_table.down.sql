-- Add down migration script here
DROP INDEX public.audit_logs_instance_id_idx;
DROP TABLE IF EXISTS public.audit_log_entries;
