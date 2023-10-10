---------------------------------------------------------------------------------------------------
-- Optional table schema
---------------------------------------------------------------------------------------------------
-- Drop database schema
DROP OWNED BY fastrue_admin;
DROP ROLE IF EXISTS fastrue_admin;
DROP SCHEMA IF EXISTS auth;

-- Initialize database schema
CREATE USER fastrue_admin NOINHERIT CREATEROLE LOGIN NOREPLICATION PASSWORD 'securedb';
CREATE SCHEMA auth AUTHORIZATION fastrue_admin;
GRANT CREATE ON DATABASE postgres TO fastrue_admin;
ALTER USER fastrue_admin SET search_path = 'auth';

---------------------------------------------------------------------------------------------------
-- Auth function
---------------------------------------------------------------------------------------------------
-- Drop auth_functions
DROP FUNCTION IF EXISTS public.jwt();

-- Create auth_functions
CREATE OR REPLACE FUNCTION public.jwt()
RETURNS jsonb
LANGUAGE sql stable
AS $$
  select coalesce(
    nullif(current_setting('request.jwt.claim', true), ''),
    nullif(current_setting('request.jwt.claims', true), '')
  )::jsonb
$$;

---------------------------------------------------------------------------------------------------
-- Users table
---------------------------------------------------------------------------------------------------
-- Drop users table
DROP INDEX IF EXISTS public.users_email_partial_key;
DROP INDEX IF EXISTS public.confirmation_token_idx;
DROP INDEX IF EXISTS public.recovery_token_idx;
DROP INDEX IF EXISTS public.email_change_token_current_idx;
DROP INDEX IF EXISTS public.email_change_token_new_idx;
DROP INDEX IF EXISTS public.reauthentication_token_idx;
DROP TABLE IF EXISTS public.users

-- Create users table
CREATE TABLE public.users (
  id UUID NOT NULL UNIQUE,
  uid varchar(25) NOT NULL UNIQUE,
  aud varchar(255) NULL,
  "role" varchar(255) NULL,
  email varchar(255) NULL UNIQUE,
  email_confirmed_at timestamptz NULL,
  email_change_token_new varchar(255) NULL,
  email_change varchar(255) NULL,
  email_change_sent_at timestamptz NULL,
  email_change_token_current varchar(255) NULL DEFAULT '',
  email_change_confirm_status smallint DEFAULT 0 CHECK (email_change_confirm_status >= 0 AND email_change_confirm_status <= 2),
  phone text NULL UNIQUE DEFAULT NULL,
  phone_confirmed_at timestamptz NULL DEFAULT NULL,
  phone_change text NULL DEFAULT '',
  phone_change_token varchar(255) NULL DEFAULT '',
  phone_change_sent_at timestamptz NULL DEFAULT NULL,
  invited_at timestamptz NULL,
  confirmation_token varchar(255) NULL,
  confirmation_sent_at timestamptz NULL,
  recovery_token varchar(255) NULL,
  recovery_sent_at timestamptz NULL,
  reauthentication_token varchar(255) NULL DEFAULT '',
  reauthentication_sent_at timestamptz NULL DEFAULT NULL,
  last_sign_in_at timestamptz NULL,
  raw_app_meta_data jsonb NULL,
  raw_user_meta_data jsonb NULL,
  is_super_admin BOOL NULL,
  is_sso_user BOOL NOT NULL DEFAULT FALSE,
  confirmed_at timestamptz GENERATED ALWAYS AS (LEAST (public.users.email_confirmed_at, public.users.phone_confirmed_at)) STORED,
  banned_until timestamptz NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  deleted_at timestamptz NULL,
  CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE UNIQUE INDEX confirmation_token_idx ON public.users USING btree (confirmation_token) WHERE confirmation_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX recovery_token_idx ON public.users USING btree (recovery_token) WHERE recovery_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX email_change_token_current_idx ON public.users USING btree (email_change_token_current) WHERE email_change_token_current !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX email_change_token_new_idx ON public.users USING btree (email_change_token_new) WHERE email_change_token_new !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX reauthentication_token_idx ON public.users USING btree (reauthentication_token) WHERE reauthentication_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX users_email_partial_key ON public.users (email) WHERE (is_sso_user = false);

COMMENT ON TABLE public.users is 'Auth: Stores user login data within a secure schema.';
COMMENT ON COLUMN public.users.is_sso_user is 'Auth: Set this column to true when the account comes from SSO. These accounts can have duplicate emails.';
COMMENT ON INDEX public.users_email_partial_key is 'Auth: A partial unique index that applies only when is_sso_user is false';

---------------------------------------------------------------------------------------------------
-- Passwords table
---------------------------------------------------------------------------------------------------
-- Drop passwords table
DROP INDEX IF EXISTS public.passwords_user_id_idx;
DROP TABLE IF EXISTS public.passwords;

-- Create passwords table
CREATE TABLE public.passwords (
  id UUID NOT NULL,
  user_id UUID NOT NULL UNIQUE,
  encrypted_password varchar(255) NOT NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT passwords_pkey PRIMARY KEY (id),
  CONSTRAINT passwords_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE INDEX passwords_user_id_idx on public.passwords (user_id);
COMMENT ON TABLE public.passwords is 'Auth: Stores encrypted password associated to a user.';

---------------------------------------------------------------------------------------------------
-- Session table
---------------------------------------------------------------------------------------------------
-- Drop sessions table
DROP INDEX IF EXISTS public.sessions_user_id_idx;
DROP TABLE IF EXISTS public.sessions;
DROP TYPE IF EXISTS aal_level;

-- Create sessions table
CREATE TYPE aal_level as ENUM('aal1', 'aal2', 'aal3');
CREATE TABLE public.sessions (
  id UUID NOT NULL,
  user_id UUID NOT NULL,
  factor_id UUID NULL,
  aal aal_level NULL,
  not_after timestamptz,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT sessions_pkey PRIMARY KEY (id),
  CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE INDEX sessions_user_id_idx on public.sessions (user_id);
CREATE INDEX sessions_not_after_idx ON public.sessions (not_after desc);
COMMENT ON TABLE public.sessions is 'Auth: Stores session data associated to a user.';
COMMENT ON COLUMN public.sessions.not_after is 'Auth: Not after is a nullable column that contains a timestamp after which the session should be regarded as expired.';

---------------------------------------------------------------------------------------------------
-- Refresh Token table
---------------------------------------------------------------------------------------------------
-- Drop refresh_tokens table
DROP INDEX IF EXISTS public.refresh_tokens_token_idx;
DROP INDEX IF EXISTS public.refresh_tokens_parent_idx;
DROP INDEX IF EXISTS public.refresh_tokens_session_id_revoked_idx;
DROP TABLE IF EXISTS public.refresh_tokens;

-- Create refresh_tokens table
CREATE TABLE public.refresh_tokens (
  id UUID NOT NULL,
  parent varchar(255) NULL,
  "token" varchar(255) NULL,
  user_id varchar(255) NULL,
  revoked BOOL NULL,
  session_id UUID NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT refresh_tokens_pkey PRIMARY KEY (id),
  CONSTRAINT refresh_tokens_token_unique UNIQUE (token),
  CONSTRAINT refresh_tokens_session_id_fkey FOREIGN KEY (session_id) REFERENCES public.sessions(id) on delete cascade
);

CREATE INDEX refresh_tokens_token_idx ON public.refresh_tokens USING btree (token);
CREATE INDEX refresh_tokens_parent_idx ON public.refresh_tokens USING btree (parent);
CREATE INDEX refresh_tokens_session_id_revoked_idx on public.refresh_tokens (session_id, revoked);
CREATE INDEX refresh_tokens_updated_at_idx ON public.refresh_tokens (updated_at desc);

COMMENT ON TABLE public.refresh_tokens is 'Auth: Store of tokens used to refresh JWT tokens once they expire.';

---------------------------------------------------------------------------------------------------
-- Audit log table
---------------------------------------------------------------------------------------------------
-- Drop table audit_log_entries
DROP TABLE IF EXISTS public.audit_log_entries;

-- Create table audit_log_entries
CREATE TABLE public.audit_log_entries (
  id UUID NOT NULL,
  payload json NULL,
  ip_address varchar(64) NOT NULL DEFAULT '',
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT audit_log_entries_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE public.audit_log_entries is 'Auth: Audit trail for user actions.';

---------------------------------------------------------------------------------------------------
-- Identities table
---------------------------------------------------------------------------------------------------
-- Drop tables identities
DROP INDEX IF EXISTS public.identities_user_id_idx;
DROP INDEX IF EXISTS public.identities_email_idx;
DROP TABLE IF EXISTS public.identities;

-- Create tables identities
CREATE TABLE public.identities (
    id text NOT NULL,
    user_id UUID NOT NULL,
    identity_data jsonb NOT NULL,
    provider text NOT NULL,
    -- email text GENERATED ALWAYS AS (lower(identity_data->>'email')) STORED,
    email text,
    last_sign_in_at timestamptz NULL,
    created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
    updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
    CONSTRAINT identities_pkey PRIMARY KEY (provider, id),
    CONSTRAINT identities_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);
CREATE INDEX identities_user_id_idx ON public.identities using btree (user_id);
COMMENT ON TABLE public.identities is 'Auth: Stores identities associated to a user.';
COMMENT ON COLUMN public.identities.email is 'Auth: Email is a generated column that REFERENCES the optional email property in the identity_data';

--- This part incompatible with CockroachDB (issue: operator classes)
CREATE INDEX identities_email_idx on public.identities (email text_pattern_ops);
COMMENT ON INDEX public.identities_email_idx is 'Auth: Ensures indexed queries on the email column';

---------------------------------------------------------------------------------------------------
-- MFA tables
---------------------------------------------------------------------------------------------------
-- Drop table mfa
DROP INDEX IF EXISTS public.user_id_created_at_idx;
DROP INDEX IF EXISTS public.factor_id_created_at_idx;
DROP TABLE IF EXISTS public.mfa_challenges;
DROP INDEX IF EXISTS public.mfa_factors_user_friendly_name_unique;
DROP TABLE IF EXISTS public.mfa_factors;
DROP TABLE IF EXISTS public.mfa_amr_claims;
DROP TYPE IF EXISTS factor_type;
DROP TYPE IF EXISTS factor_status;

-- Create table mfa
CREATE TYPE factor_type AS ENUM('totp', 'webauthn');
CREATE TYPE factor_status AS ENUM('unverified', 'verified');

CREATE TABLE public.mfa_factors(
  id UUID NOT NULL,
  user_id UUID NOT NULL,
  friendly_name text NULL,
  factor_type factor_type NOT NULL,
  status factor_status NOT NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  secret text NULL,
  CONSTRAINT mfa_factors_pkey PRIMARY KEY(id),
  CONSTRAINT mfa_factors_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) on delete cascade
);

CREATE UNIQUE INDEX mfa_factors_user_friendly_name_unique on public.mfa_factors (friendly_name, user_id) WHERE TRIM(friendly_name) <> '';
CREATE INDEX mfa_factors_user_id_idx ON public.mfa_factors(user_id);
COMMENT ON TABLE public.mfa_factors is 'auth: stores metadata about factors';

CREATE TABLE public.mfa_challenges(
  id UUID NOT NULL,
  factor_id UUID NOT NULL,
  created_at timestamptz NOT NULL,
  verified_at timestamptz  NULL,
  ip_address  inet NOT NULL,
  CONSTRAINT mfa_challenges_pkey PRIMARY KEY (id),
  CONSTRAINT mfa_challenges_auth_factor_id_fkey FOREIGN KEY (factor_id) REFERENCES public.mfa_factors(id) on delete cascade
);
CREATE INDEX mfa_challenge_created_at_idx ON public.mfa_challenges (created_at desc);
COMMENT ON TABLE public.mfa_challenges is 'auth: stores metadata about challenge requests made';

create table if not exists public.mfa_amr_claims(
  id UUID NOT NULL,
  session_id UUID NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  authentication_method text NOT NULL,
  CONSTRAINT amr_id_pk PRIMARY KEY(id),
  CONSTRAINT mfa_amr_claims_session_id_authentication_method_pkey unique(session_id, authentication_method),
  CONSTRAINT mfa_amr_claims_session_id_fkey FOREIGN KEY(session_id) REFERENCES public.sessions(id) on delete cascade
);
CREATE INDEX user_id_created_at_idx on public.sessions (user_id, created_at);
CREATE INDEX factor_id_created_at_idx on public.mfa_factors (user_id, created_at);
COMMENT ON TABLE public.mfa_amr_claims is 'auth: stores authenticator method REFERENCE claims for multi factor authentication';

---------------------------------------------------------------------------------------------------
-- Flow state table
---------------------------------------------------------------------------------------------------
-- Drop flow_state table
DROP INDEX IF EXISTS public.idx_auth_code;
DROP INDEX IF EXISTS public.idx_user_id_auth_method;
DROP TABLE IF EXISTS public.flow_state;
DROP TYPE IF EXISTS code_challenge_method;

-- Create flow_state table
CREATE TYPE code_challenge_method AS ENUM('s256', 'plain');
CREATE TABLE public.flow_state(
  id UUID PRIMARY KEY,
  user_id UUID NULL,
  auth_code text NOT NULL,
  authentication_method text NOT NULL,
  code_challenge_method code_challenge_method NOT NULL,
  code_challenge text NOT NULL,
  provider_type text NOT NULL,
  provider_access_token text NULL,
  provider_refresh_token text NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL
);
CREATE INDEX idx_auth_code ON public.flow_state(auth_code);
CREATE INDEX idx_user_id_auth_method ON public.flow_state (user_id, authentication_method);
CREATE INDEX flow_state_created_at_idx ON public.flow_state (created_at desc);
COMMENT ON TABLE public.flow_state is 'stores metadata for oauth provider logins';

---------------------------------------------------------------------------------------------------
-- SAML tables
---------------------------------------------------------------------------------------------------
-- Drop table saml
DROP INDEX IF EXISTS public.sso_domains_sso_provider_id_idx;
DROP INDEX IF EXISTS public.sso_domains_domain_idx;
DROP TABLE IF EXISTS public.sso_domains;

DROP INDEX IF EXISTS public.saml_providers_sso_provider_id_idx;
DROP TABLE IF EXISTS public.saml_providers

DROP INDEX IF EXISTS public.saml_relay_states_sso_provider_id_idx;
DROP INDEX IF EXISTS public.saml_relay_states_for_email_idx;
DROP TABLE IF EXISTS public.saml_relay_states;

DROP INDEX IF EXISTS public.sso_providers_resource_id_idx;
DROP TABLE IF EXISTS public.sso_providers

-- Create table saml
CREATE TABLE public.sso_providers (
  id UUID NOT NULL,
  resource_id text NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT "resource_id not empty" CHECK (resource_id = null or char_length(resource_id) > 0)
);
COMMENT ON TABLE public.sso_providers is 'Auth: Manages SSO identity provider information; see saml_providers for SAML.';
COMMENT ON COLUMN public.sso_providers.resource_id is 'Auth: Uniquely identifies a SSO provider according to a user-chosen resource ID (case insensitive), useful in infrastructure as code.';
CREATE UNIQUE INDEX sso_providers_resource_id_idx on public.sso_providers (lower(resource_id));

CREATE TABLE public.sso_domains (
  id UUID NOT NULL,
  sso_provider_id UUID NOT NULL,
  domain text NOT NULL,
  created_at timestamptz NULL,
  updated_at timestamptz NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sso_provider_id) REFERENCES public.sso_providers (id) ON DELETE CASCADE,
  CONSTRAINT "domain not empty" CHECK (char_length(domain) > 0)
);
CREATE INDEX sso_domains_sso_provider_id_idx on public.sso_domains (sso_provider_id);
CREATE UNIQUE INDEX sso_domains_domain_idx on public.sso_domains (lower(domain));
COMMENT ON TABLE public.sso_domains is 'Auth: Manages SSO email address domain mapping to an SSO Identity Provider.';

CREATE TABLE public.saml_providers (
  id UUID NOT NULL,
  sso_provider_id UUID NOT NULL,
  entity_id text NOT NULL unique,
  metadata_xml text NOT NULL,
  metadata_url text NULL,
  attribute_mapping jsonb NULL,
  created_at timestamptz NULL,
  updated_at timestamptz NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sso_provider_id) REFERENCES public.sso_providers (id) ON DELETE CASCADE,
  CONSTRAINT "metadata_xml not empty" CHECK (char_length(metadata_xml) > 0),
  CONSTRAINT "metadata_url not empty" CHECK (metadata_url = null or char_length(metadata_url) > 0),
  CONSTRAINT "entity_id not empty" CHECK (char_length(entity_id) > 0)
);
CREATE INDEX saml_providers_sso_provider_id_idx on public.saml_providers (sso_provider_id);
COMMENT ON TABLE public.saml_providers is 'Auth: Manages SAML Identity Provider connections.';

CREATE TABLE public.saml_relay_states (
  id UUID NOT NULL,
  sso_provider_id UUID NOT NULL,
  flow_state_id UUID,
  request_id text NOT NULL,
  for_email text NULL,
  redirect_to text NULL,
  from_ip_address inet NULL,
  created_at timestamptz NULL,
  updated_at timestamptz NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sso_provider_id) REFERENCES public.sso_providers (id) ON DELETE CASCADE,
  FOREIGN KEY (flow_state_id) REFERENCES public.flow_state (id) ON DELETE CASCADE,
  CONSTRAINT "request_id not empty" CHECK(char_length(request_id) > 0)
);
CREATE INDEX saml_relay_states_sso_provider_id_idx on public.saml_relay_states (sso_provider_id);
CREATE INDEX saml_relay_states_for_email_idx on public.saml_relay_states (for_email);
CREATE INDEX saml_relay_states_created_at_idx ON public.saml_relay_states (created_at desc);
COMMENT ON TABLE public.saml_relay_states is 'Auth: Contains SAML Relay State information for each Service Provider initiated login.';
