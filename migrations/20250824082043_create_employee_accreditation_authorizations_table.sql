-- Migration: Create employee accreditation authorizations table
CREATE TABLE IF NOT EXISTS public."employee_accreditation_authorizations" (
    pk_employee_accreditation_authorization_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fk_recipient_employee_id UUID NOT NULL,
    fk_employee_level_id INTEGER NOT NULL,
    fk_authorizing_employee_id UUID,
    start_at TIMESTAMP WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_recipient_employee_id
    FOREIGN KEY (fk_recipient_employee_id)
    REFERENCES employees(pk_employee_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_employee_level_id
    FOREIGN KEY (fk_employee_level_id)
    REFERENCES employee_levels(pk_employee_level_id)
    ON DELETE CASCADE,
    CONSTRAINT fk_authorizing_employee_id
    FOREIGN KEY (fk_authorizing_employee_id)
    REFERENCES employees(pk_employee_id)
    ON DELETE CASCADE
);
