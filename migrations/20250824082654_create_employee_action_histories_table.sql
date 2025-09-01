-- Migration: Create employee action histories table
CREATE TABLE IF NOT EXISTS public."employee_action_histories" (
    fk_employee_id UUID NOT NULL,
    fk_employee_authorization_type_id INTEGER NOT NULL,
    fk_entity_id UUID NOT NULL,
    fk_entity_type "EntityType" NOT NULL,
    description JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    CONSTRAINT employee_action_history_pkey PRIMARY KEY (fk_employee_id, fk_employee_authorization_type_id, created_at),

    CONSTRAINT fk_employee_id
    FOREIGN KEY (fk_employee_id)
    REFERENCES employees(pk_employee_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_employee_authorization_type_id
    FOREIGN KEY (fk_employee_authorization_type_id)
    REFERENCES employee_authorization_types(pk_employee_authorization_type_id)
    ON DELETE CASCADE
);