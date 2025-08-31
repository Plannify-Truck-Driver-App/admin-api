-- Migration: Create employee authorization derogations table
CREATE TABLE IF NOT EXISTS public."employee_authorization_derogations" (
    pk_employee_authorization_derogation_id SERIAL PRIMARY KEY,
    fk_recipient_employee_id UUID NOT NULL,
    fk_employee_authorization_type_id INTEGER NOT NULL,
    fk_authorizing_employee_id UUID NOT NULL,
    derogation_reason VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    start_at TIMESTAMP WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP WITH TIME ZONE NOT NULL,

    CONSTRAINT fk_recipient_employee_id
    FOREIGN KEY (fk_recipient_employee_id)
    REFERENCES employees(pk_employee_id),
    CONSTRAINT fk_employee_authorization_type_id
    FOREIGN KEY (fk_employee_authorization_type_id)
    REFERENCES employee_authorization_types(pk_employee_authorization_type_id),
    CONSTRAINT fk_authorizing_employee_id
    FOREIGN KEY (fk_authorizing_employee_id)
    REFERENCES employees(pk_employee_id)
);