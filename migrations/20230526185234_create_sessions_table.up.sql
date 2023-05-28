CREATE TYPE aal_level as ENUM('aal1', 'aal2', 'aal3');
CREATE TABLE IF NOT EXISTS public.sessions (
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

CREATE INDEX IF NOT EXISTS sessions_user_id_idx on public.sessions (user_id);
COMMENT ON TABLE public.sessions is 'Auth: Stores session data associated to a user.';
COMMENT ON COLUMN public.sessions.not_after is 'Auth: Not after is a nullable column that contains a timestamp after which the session should be regarded as expired.';
