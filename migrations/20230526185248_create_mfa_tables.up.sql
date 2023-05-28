CREATE TYPE factor_type AS ENUM('totp', 'webauthn');
CREATE TYPE factor_status AS ENUM('unverified', 'verified');

CREATE TABLE IF NOT EXISTS public.mfa_factors(
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

CREATE UNIQUE INDEX IF NOT EXISTS mfa_factors_user_friendly_name_unique on public.mfa_factors (friendly_name, user_id) WHERE TRIM(friendly_name) <> '';
COMMENT ON TABLE public.mfa_factors is 'auth: stores metadata about factors';

CREATE TABLE IF NOT EXISTS public.mfa_challenges(
  id UUID NOT NULL,
  factor_id UUID NOT NULL,
  created_at timestamptz NOT NULL,
  verified_at timestamptz  NULL,
  ip_address  inet NOT NULL,
  CONSTRAINT mfa_challenges_pkey PRIMARY KEY (id),
  CONSTRAINT mfa_challenges_auth_factor_id_fkey FOREIGN KEY (factor_id) REFERENCES public.mfa_factors(id) on delete cascade
);
COMMENT ON TABLE public.mfa_challenges is 'auth: stores metadata about challenge requests made';

CREATE TABLE IF NOT EXISTS public.mfa_amr_claims(
  id UUID NOT NULL,
  session_id UUID NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NOT NULL,
  authentication_method text NOT NULL,
  CONSTRAINT amr_id_pk PRIMARY KEY(id),
  CONSTRAINT mfa_amr_claims_session_id_authentication_method_pkey unique(session_id, authentication_method),
  CONSTRAINT mfa_amr_claims_session_id_fkey FOREIGN KEY(session_id) REFERENCES public.sessions(id) on delete cascade
);
CREATE INDEX IF NOT EXISTS user_id_created_at_idx on public.sessions (user_id, created_at);
CREATE INDEX IF NOT EXISTS factor_id_created_at_idx on public.mfa_factors (user_id, created_at);
COMMENT ON TABLE public.mfa_amr_claims is 'auth: stores authenticator method REFERENCE claims for multi factor authentication';
