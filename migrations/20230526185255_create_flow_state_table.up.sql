CREATE TYPE code_challenge_method AS ENUM('s256', 'plain');
CREATE TABLE IF NOT EXISTS public.flow_state(
  id UUID PRIMARY KEY,
  user_id UUID NULL,
  auth_code text NOT NULL,
  code_challenge_method code_challenge_method NOT NULL,
  code_challenge text NOT NULL,
  provider_type text NOT NULL,
  provider_access_token text NULL,
  provider_refresh_token text NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL
);
CREATE INDEX idx_auth_code ON public.flow_state(auth_code);
COMMENT ON TABLE public.flow_state is 'stores metadata for oauth provider logins';
