CREATE TABLE IF NOT EXISTS public.sso_providers (
  id UUID NOT NULL,
  resource_id text NULL,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  PRIMARY KEY (id),
  CONSTRAINT "resource_id not empty" CHECK (resource_id = null or char_length(resource_id) > 0)
);
COMMENT ON TABLE public.sso_providers is 'Auth: Manages SSO identity provider information; see saml_providers for SAML.';
COMMENT ON COLUMN public.sso_providers.resource_id is 'Auth: Uniquely identifies a SSO provider according to a user-chosen resource ID (case insensitive), useful in infrastructure as code.';
CREATE UNIQUE INDEX IF NOT EXISTS sso_providers_resource_id_idx on public.sso_providers (lower(resource_id));

CREATE TABLE IF NOT EXISTS public.sso_domains (
  id UUID NOT NULL,
  sso_provider_id UUID NOT NULL,
  domain text NOT NULL,
  created_at timestamptz NULL,
  updated_at timestamptz NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sso_provider_id) REFERENCES public.sso_providers (id) ON DELETE CASCADE,
  CONSTRAINT "domain not empty" CHECK (char_length(domain) > 0)
);
CREATE INDEX IF NOT EXISTS sso_domains_sso_provider_id_idx on public.sso_domains (sso_provider_id);
CREATE UNIQUE INDEX IF NOT EXISTS sso_domains_domain_idx on public.sso_domains (lower(domain));
COMMENT ON TABLE public.sso_domains is 'Auth: Manages SSO email address domain mapping to an SSO Identity Provider.';

CREATE TABLE IF NOT EXISTS public.saml_providers (
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
CREATE INDEX IF NOT EXISTS saml_providers_sso_provider_id_idx on public.saml_providers (sso_provider_id);
COMMENT ON TABLE public.saml_providers is 'Auth: Manages SAML Identity Provider connections.';

CREATE TABLE IF NOT EXISTS public.saml_relay_states (
  id UUID NOT NULL,
  sso_provider_id UUID NOT NULL,
  request_id text NOT NULL,
  for_email text NULL,
  redirect_to text NULL,
  from_ip_address inet NULL,
  created_at timestamptz NULL,
  updated_at timestamptz NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (sso_provider_id) REFERENCES public.sso_providers (id) ON DELETE CASCADE,
  CONSTRAINT "request_id not empty" CHECK(char_length(request_id) > 0)
);
CREATE INDEX IF NOT EXISTS saml_relay_states_sso_provider_id_idx on public.saml_relay_states (sso_provider_id);
CREATE INDEX IF NOT EXISTS saml_relay_states_for_email_idx on public.saml_relay_states (for_email);
COMMENT ON TABLE public.saml_relay_states is 'Auth: Contains SAML Relay State information for each Service Provider initiated login.';
