INSERT INTO employee_authorization_categories (pk_employee_authorization_category_id, name_code, entity_type, category_index) VALUES (1, 'USER_INFORMATIONS', 'USER', 1);

INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (1, 1, 'USER_GLOBAL_INFORMATIONS', 1);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (2, 1, 'USER_MAIL_INFORMATIONS', 2);
INSERT INTO employee_authorizations (pk_employee_authorization_id, fk_employee_authorization_category_id, feature_code, authorization_index) VALUES (3, 1, 'USER_SUSPENSION_INFORMATIONS', 3);

INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (1, 1, 'R', 'Read all informations');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (2, 1, 'C', 'Create a new user');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (3, 1, 'U', 'Update a user');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (4, 1, 'D', 'Delete a user');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (5, 2, 'R', 'Read all user mails');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (6, 2, 'C', 'Send a mail to a user');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (7, 2, 'U', 'Update a sent mail');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (8, 2, 'D', 'Delete a sent mail');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (9, 3, 'R', 'Read all user suspensions');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (10, 3, 'C', 'Create a new user suspension');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (11, 3, 'U', 'Update a user suspension');
INSERT INTO employee_authorization_types (pk_employee_authorization_type_id, fk_employee_authorization_id, crud_type, description) VALUES (12, 3, 'D', 'Delete a user suspension');

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

INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 1);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 2);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 3);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 5);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 6);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 9);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 10);
INSERT INTO link_employee_authorization (fk_employee_level_id, fk_employee_authorization_type_id) VALUES (2, 11);

INSERT INTO employee_accreditation_authorizations (fk_recipient_employee_id, fk_employee_level_id, fk_authorizing_employee_id, start_at, end_at) VALUES ('45050fca-e4b1-4439-800c-885dcc93c840', 1, '45050fca-e4b1-4439-800c-885dcc93c840', '2025-01-01', '2026-01-01');