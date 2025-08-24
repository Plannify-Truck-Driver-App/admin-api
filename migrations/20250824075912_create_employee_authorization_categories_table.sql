-- Migration: Create employee authorization categories table
CREATE TABLE IF NOT EXISTS public."employee_authorization_categories" (
    pk_employee_authorization_category_id SERIAL PRIMARY KEY,
    name_code VARCHAR(255) NOT NULL UNIQUE,
    entity_type "EntityType" NOT NULL DEFAULT 'DRIVER',
    category_index INTEGER NOT NULL
);