-- Create users table
CREATE TABLE IF NOT EXISTS public.users (
	instance_id uuid NULL,
	id uuid NOT NULL,
	aud varchar(255) NULL,
	"role" varchar(255) NULL,
	email varchar(255) NULL,
	encrypted_password varchar(255) NULL,
	confirmed_at timestamptz NULL,
	invited_at timestamptz NULL,
	confirmation_token varchar(255) NULL,
	confirmation_sent_at timestamptz NULL,
	recovery_token varchar(255) NULL,
	recovery_sent_at timestamptz NULL,
	email_change_token varchar(255) NULL,
	email_change varchar(255) NULL,
	email_change_sent_at timestamptz NULL,
	last_sign_in_at timestamptz NULL,
	raw_app_meta_data jsonb NULL,
	raw_user_meta_data jsonb NULL,
	is_super_admin bool NULL,
	created_at timestamptz NULL,
	updated_at timestamptz NULL,
	CONSTRAINT users_pkey PRIMARY KEY (id)
);
CREATE INDEX users_instance_id_email_idx ON public.users USING btree (instance_id, email);
CREATE INDEX users_instance_id_idx ON public.users USING btree (instance_id);

-- Create instances table
CREATE TABLE public.instances (
	id uuid NOT NULL,
	uuid uuid NULL,
	raw_base_config text NULL,
	created_at timestamptz NULL,
	updated_at timestamptz NULL,
	CONSTRAINT instances_pkey PRIMARY KEY (id)
);

-- Create refresh_tokens table
CREATE TABLE IF NOT EXISTS public.refresh_tokens (
	instance_id uuid NULL,
	id bigserial NOT NULL,
	"token" varchar(255) NULL,
	"user_id" varchar(255) NULL,
	revoked bool NULL,
	created_at timestamptz NULL,
	updated_at timestamptz NULL,
	CONSTRAINT refresh_tokens_pkey PRIMARY KEY (id)
);
CREATE INDEX refresh_tokens_instance_id_idx ON public.refresh_tokens USING btree (instance_id);
CREATE INDEX refresh_tokens_instance_id_user_id_idx ON public.refresh_tokens USING btree (instance_id, user_id);
CREATE INDEX refresh_tokens_token_idx ON public.refresh_tokens USING btree (token);

-- Create audit_log_entries table
CREATE TABLE public.audit_log_entries (
	instance_id uuid NULL,
	id uuid NOT NULL,
	payload json NULL,
	created_at timestamptz NULL,
	CONSTRAINT audit_log_entries_pkey PRIMARY KEY (id)
);
CREATE INDEX audit_logs_instance_id_idx ON public.audit_log_entries USING btree (instance_id);
