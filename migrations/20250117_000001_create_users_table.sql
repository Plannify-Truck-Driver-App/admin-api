-- Migration: Create users table
CREATE TABLE IF NOT EXISTS public."user" (
    pk_user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    firstname VARCHAR(255) NOT NULL,
    lastname VARCHAR(255) NOT NULL,
    gender VARCHAR(1),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    phone_number VARCHAR(20),
    is_searchable BOOLEAN DEFAULT true NOT NULL,
    allow_request_professional_agreement BOOLEAN DEFAULT false NOT NULL,
    language VARCHAR(10) DEFAULT 'fr' NOT NULL,
    rest_json JSONB,
    mail_preferences INTEGER DEFAULT 0 NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    verified_at TIMESTAMP WITH TIME ZONE,
    last_login_at TIMESTAMP WITH TIME ZONE,
    deactivated_at TIMESTAMP WITH TIME ZONE
);