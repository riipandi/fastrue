DROP INDEX IF EXISTS public.refresh_tokens_token_idx;
DROP INDEX IF EXISTS public.refresh_tokens_parent_idx;
DROP INDEX IF EXISTS public.refresh_tokens_session_id_revoked_idx;
DROP TABLE IF EXISTS public.refresh_tokens;
