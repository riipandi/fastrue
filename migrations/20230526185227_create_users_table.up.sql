CREATE TABLE IF NOT EXISTS public.users (
  id uuid NOT NULL UNIQUE,
  uid varchar(24) NOT NULL UNIQUE,
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

CREATE UNIQUE INDEX IF NOT EXISTS confirmation_token_idx ON public.users USING btree (confirmation_token) WHERE confirmation_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS recovery_token_idx ON public.users USING btree (recovery_token) WHERE recovery_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS email_change_token_current_idx ON public.users USING btree (email_change_token_current) WHERE email_change_token_current !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS email_change_token_new_idx ON public.users USING btree (email_change_token_new) WHERE email_change_token_new !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS reauthentication_token_idx ON public.users USING btree (reauthentication_token) WHERE reauthentication_token !~ '^[0-9 ]*$';
CREATE UNIQUE INDEX IF NOT EXISTS users_email_partial_key ON public.users (email) WHERE (is_sso_user = false);

COMMENT ON TABLE public.users is 'Auth: Stores user login data within a secure schema.';
COMMENT ON COLUMN public.users.is_sso_user is 'Auth: Set this column to true when the account comes from SSO. These accounts can have duplicate emails.';
COMMENT ON INDEX public.users_email_partial_key is 'Auth: A partial unique index that applies only when is_sso_user is false';
