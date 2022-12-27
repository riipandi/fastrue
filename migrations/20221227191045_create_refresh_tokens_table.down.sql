-- Add down migration script here
DROP INDEX public.refresh_tokens_instance_id_idx;
DROP INDEX public.refresh_tokens_instance_id_user_id_idx;
DROP INDEX public.refresh_tokens_token_idx;
DROP TABLE IF EXISTS public.refresh_tokens;
