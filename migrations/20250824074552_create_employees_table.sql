-- Migration: Create employees table
CREATE TABLE IF NOT EXISTS public."employees" (
    pk_employee_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    firstname VARCHAR(255) NOT NULL,
    lastname VARCHAR(255) NOT NULL,
    gender VARCHAR(1),
    personal_email VARCHAR(255) NOT NULL,
    login_password_hash TEXT NOT NULL,
    phone_number VARCHAR(20),
    professional_email VARCHAR(255) NOT NULL UNIQUE,
    professional_email_password VARCHAR(40) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    last_login_at TIMESTAMP WITH TIME ZONE,
    deactivated_at TIMESTAMP WITH TIME ZONE
);
