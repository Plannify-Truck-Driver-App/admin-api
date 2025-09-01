-- Migration: Create employee authorizations table
CREATE TABLE IF NOT EXISTS public."employee_authorizations" (
    pk_employee_authorization_id SERIAL PRIMARY KEY,
    fk_employee_authorization_category_id INTEGER NOT NULL,
    feature_code VARCHAR(255) NOT NULL UNIQUE,
    authorization_index INTEGER NOT NULL,

    CONSTRAINT fk_employee_authorization_category_id
    FOREIGN KEY (fk_employee_authorization_category_id)
    REFERENCES employee_authorization_categories(pk_employee_authorization_category_id)
    ON DELETE CASCADE
);
