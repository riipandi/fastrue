DROP INDEX IF EXISTS public.sso_domains_sso_provider_id_idx;
DROP INDEX IF EXISTS public.sso_domains_domain_idx;
DROP TABLE IF EXISTS public.sso_domains;

DROP INDEX IF EXISTS public.saml_providers_sso_provider_id_idx;
DROP TABLE IF EXISTS public.saml_providers;

DROP INDEX IF EXISTS public.saml_relay_states_sso_provider_id_idx;
DROP INDEX IF EXISTS public.saml_relay_states_for_email_idx;
DROP TABLE IF EXISTS public.saml_relay_states;

DROP INDEX IF EXISTS public.sso_providers_resource_id_idx;
DROP TABLE IF EXISTS public.sso_providers;
