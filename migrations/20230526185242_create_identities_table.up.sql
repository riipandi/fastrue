CREATE TABLE IF NOT EXISTS public.identities (
    id text NOT NULL,
    user_id uuid NOT NULL,
    identity_data jsonb NOT NULL,
    provider text NOT NULL,
    email text GENERATED ALWAYS AS (lower(identity_data->>'email')) STORED,
    last_sign_in_at timestamptz NULL,
    created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
    updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
    CONSTRAINT identities_pkey PRIMARY KEY (provider, id),
    CONSTRAINT identities_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS identities_user_id_idx ON public.identities using btree (user_id);
COMMENT ON TABLE public.identities is 'Auth: Stores identities associated to a user.';
COMMENT ON COLUMN public.identities.email is 'Auth: Email is a generated column that REFERENCES the optional email property in the identity_data';

--- This part incompatible with CockroachDB (issue: operator classes)
CREATE INDEX IF NOT EXISTS identities_email_idx on public.identities (email);
COMMENT ON INDEX public.identities_email_idx is 'Auth: Ensures indexed queries on the email column';
