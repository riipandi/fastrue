CREATE TABLE IF NOT EXISTS public.passwords (
  id uuid NOT NULL DEFAULT uuid_generate_v1mc(),
  user_id uuid NOT NULL UNIQUE,
  encrypted_password varchar(255) NOT NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT passwords_pkey PRIMARY KEY (id),
  CONSTRAINT passwords_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS passwords_user_id_idx on public.passwords (user_id);
COMMENT ON TABLE public.passwords is 'Auth: Stores encrypted password associated to a user.';
