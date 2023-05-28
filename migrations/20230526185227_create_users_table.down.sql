DROP INDEX IF EXISTS public.users_email_partial_key;
DROP INDEX IF EXISTS public.confirmation_token_idx;
DROP INDEX IF EXISTS public.recovery_token_idx;
DROP INDEX IF EXISTS public.email_change_token_current_idx;
DROP INDEX IF EXISTS public.email_change_token_new_idx;
DROP INDEX IF EXISTS public.reauthentication_token_idx;
DROP TABLE IF EXISTS public.users;
