-- Migration: Create link employee authorization table
CREATE TABLE IF NOT EXISTS public."link_employee_authorization" (
    fk_employee_authorization_type_id INTEGER NOT NULL,
    fk_employee_level_id INTEGER NOT NULL,

    CONSTRAINT link_employee_authorization_pkey PRIMARY KEY (fk_employee_authorization_type_id, fk_employee_level_id),

    CONSTRAINT fk_employee_authorization_type_id
    FOREIGN KEY (fk_employee_authorization_type_id)
    REFERENCES employee_authorization_types(pk_employee_authorization_type_id),
    CONSTRAINT fk_employee_level_id
    FOREIGN KEY (fk_employee_level_id)
    REFERENCES employee_levels(pk_employee_level_id)
);
