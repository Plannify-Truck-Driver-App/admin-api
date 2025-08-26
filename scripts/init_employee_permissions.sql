INSERT INTO employee_authorization_categories (pk_employee_authorization_category_id, name_code, entity_type, category_index) VALUES (1, 'DRIVER_INFORMATIONS', 'DRIVER', 1);
INSERT INTO employee_authorization_categories (pk_employee_authorization_category_id, name_code, entity_type, category_index) VALUES (2, 'SYSTEM_INFORMATIONS', 'EMPLOYEE', 2);

INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (1, 1, 'DRIVER_GLOBAL_INFORMATIONS', 1);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (2, 1, 'DRIVER_MAIL_INFORMATIONS', 2);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (3, 1, 'DRIVER_SUSPENSION_INFORMATIONS', 3);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (4, 2, 'AUTHORIZATION_INFORMATIONS', 1);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (5, 2, 'LEVEL_INFORMATIONS', 2);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (6, 2, 'ACCREDITATION_INFORMATIONS', 3);

INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (1, 1, 'R', 'Read all driver informations');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (2, 1, 'C', 'Create a new driver');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (3, 1, 'U', 'Update a driver');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (4, 1, 'D', 'Delete a driver');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (5, 2, 'R', 'Read all driver mails');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (6, 2, 'C', 'Send a mail to a driver');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (7, 2, 'U', 'Update a sent mail');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (8, 2, 'D', 'Delete a sent mail');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (9, 3, 'R', 'Read all driver suspensions');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (10, 3, 'C', 'Create a new driver suspension');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (11, 3, 'U', 'Update a driver suspension');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (12, 3, 'D', 'Delete a driver suspension');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (13, 4, 'R', 'Read employee authorizations');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (14, 5, 'R', 'Read employee levels');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (15, 5, 'C', 'Attribute a level to an employee');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (16, 5, 'U', 'Update an employee level');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (17, 5, 'D', 'Delete an employee level');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (18, 6, 'R', 'Read employee accreditations');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (19, 6, 'C', 'Create a new employee accreditation');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (20, 6, 'U', 'Update an employee accreditation');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (21, 6, 'D', 'Delete an employee accreditation');

INSERT INTO employee_levels (pk_employee_level_id, level_index, level_label) VALUES (1, 1, 'ADMIN');
INSERT INTO employee_levels (pk_employee_level_id, level_index, level_label) VALUES (2, 2, 'SUPPORT');

INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 1);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 2);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 3);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 4);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 5);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 6);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 7);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 8);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 9);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 10);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 11);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 12);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 13);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 14);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 15);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 16);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 17);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 18);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 19);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 20);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (1, 21);

INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 1);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 2);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 3);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 5);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 6);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 9);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 10);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 11);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 13);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 14);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 18);

INSERT INTO employees (pk_employee_id, firstname, lastname, gender, personal_email, login_password_hash, phone_number, professional_email, professional_email_password) VALUES ('2e6180da-b376-46df-8043-3b25d7e8be6e', 'Baptiste', 'Bronsin', 'M', 'baptiste.bronsin@outlook.com', '$2b$12$303SJbhjc5y/EouHAgoRkeq70UD3.JqzKp8b5C1ISMvr8ZcJcjPXK', null, 'baptiste.bronsin@plannify.be', 'plannify');

INSERT INTO employee_accreditation_authorizations (fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at) VALUES ('2e6180da-b376-46df-8043-3b25d7e8be6e', 1, null, '2025-01-01', '2026-01-01');