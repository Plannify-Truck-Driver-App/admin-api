-- Migration: Create employee authorization types table
CREATE TABLE IF NOT EXISTS public."employee_authorization_types" (
    pk_employee_authorization_type_id SERIAL PRIMARY KEY,
    fk_employee_authorization_id INTEGER NOT NULL,
    crud_type "CrudType" DEFAULT 'R',
    description VARCHAR(500) NOT NULL,

    CONSTRAINT fk_employee_authorization_id
    FOREIGN KEY (fk_employee_authorization_id)
    REFERENCES employee_authorizations(pk_employee_authorization_id)
);