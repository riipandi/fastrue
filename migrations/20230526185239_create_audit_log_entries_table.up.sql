CREATE TABLE public.audit_log_entries (
  id uuid NOT NULL DEFAULT uuid_generate_v1mc(),
  payload json NULL,
  ip_address varchar(64) NOT NULL DEFAULT '',
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  CONSTRAINT audit_log_entries_pkey PRIMARY KEY (id)
);
COMMENT ON TABLE public.audit_log_entries is 'Auth: Audit trail for user actions.';
