DROP INDEX IF EXISTS public.user_id_created_at_idx;
DROP INDEX IF EXISTS public.factor_id_created_at_idx;
DROP TABLE IF EXISTS public.mfa_challenges;
DROP INDEX IF EXISTS public.mfa_factors_user_friendly_name_unique;
DROP TABLE IF EXISTS public.mfa_factors;
DROP TABLE IF EXISTS public.mfa_amr_claims;
DROP TYPE IF EXISTS factor_type;
DROP TYPE IF EXISTS factor_status;
