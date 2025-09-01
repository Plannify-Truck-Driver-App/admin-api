-- Migration: Create driver workdays table
CREATE TABLE IF NOT EXISTS public."workdays" (
    date DATE NOT NULL,
    fk_driver_id UUID NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME,
    rest_time TIME NOT NULL,
    overnight_rest BOOLEAN NOT NULL,

    PRIMARY KEY (date, fk_driver_id),
    CONSTRAINT fk_driver_id
    FOREIGN KEY (fk_driver_id)
    REFERENCES drivers (pk_driver_id)
    ON DELETE CASCADE
);