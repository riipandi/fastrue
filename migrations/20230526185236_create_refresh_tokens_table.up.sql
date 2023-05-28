CREATE TABLE IF NOT EXISTS public.refresh_tokens (
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

CREATE INDEX IF NOT EXISTS refresh_tokens_token_idx ON public.refresh_tokens USING btree (token);
CREATE INDEX IF NOT EXISTS refresh_tokens_parent_idx ON public.refresh_tokens USING btree (parent);
CREATE INDEX IF NOT EXISTS refresh_tokens_session_id_revoked_idx on public.refresh_tokens (session_id, revoked);

COMMENT ON TABLE public.refresh_tokens is 'Auth: Store of tokens used to refresh JWT tokens once they expire.';
