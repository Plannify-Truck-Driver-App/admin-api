-- Migration: Create employee levels table
CREATE TABLE IF NOT EXISTS public."employee_levels" (
    pk_employee_level_id SERIAL PRIMARY KEY,
    level_index INTEGER NOT NULL,
    level_label VARCHAR(255) NOT NULL
);