#!/bin/bash

# Configuration
DB_HOST="localhost"
DB_PORT="5432"
DB_NAME="plannify_admin"
DB_USER="plannify_user"
DB_PASSWORD="plannify_password"
EMPLOYEE_EMAIL="test@example.com"

echo "🔐 Configuration des permissions des chauffeurs"
echo "=============================================="

# Vérifier que psql est installé
if ! command -v psql &> /dev/null; then
    echo "❌ psql n'est pas installé. Veuillez installer PostgreSQL client."
    exit 1
fi

echo "📋 Configuration de la base de données:"
echo "   Host: $DB_HOST"
echo "   Port: $DB_PORT"
echo "   Database: $DB_NAME"
echo "   User: $DB_USER"
echo "   Employee email: $EMPLOYEE_EMAIL"

# Demander confirmation
read -p "Voulez-vous continuer ? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Opération annulée"
    exit 1
fi

echo "🚀 Exécution du script de configuration des permissions..."

# Exécuter le script SQL
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -f scripts/init_employee_permissions.sql

if [ $? -eq 0 ]; then
    echo "✅ Permissions configurées avec succès!"
    echo ""
    echo "📊 Vérification des permissions attribuées:"
    
    # Vérifier les permissions de l'employé
    PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "
    SELECT 
        e.professional_email,
        el.level_name,
        ea.pk_employee_authorization_id as permission_code,
        ea.feature_code,
        ea.authorization_index
    FROM employees e
    JOIN employee_accreditation_authorizations eaa ON e.pk_employee_id = eaa.fk_recipient_employee_id
    JOIN employee_levels el ON eaa.fk_employee_level_id = el.pk_employee_level_id
    JOIN link_employee_authorization lea ON el.pk_employee_level_id = lea.fk_employee_level_id
    JOIN employee_authorization_types eat ON lea.fk_employee_authorization_type_id = eat.pk_employee_authorization_type_id
    JOIN employee_authorizations ea ON eat.pk_employee_authorization_type_id = ea.pk_employee_authorization_id
    WHERE e.professional_email = '$EMPLOYEE_EMAIL'
    ORDER BY ea.authorization_index;
    "
    
    echo ""
    echo "🎯 Résumé des permissions configurées:"
    echo "   - Permission 23: Lecture des chauffeurs (GET /drivers)"
    echo "   - Permission 32: Création de chauffeurs (POST /drivers)"
    echo "   - Permission 45: Modification de chauffeurs (PUT /drivers/:id)"
    echo "   - Suppression: Permissions 32 ET 45 (DELETE /drivers/:id)"
    echo ""
    echo "🧪 Vous pouvez maintenant tester l'API avec:"
    echo "   ./test_permissions_api.sh"
    
else
    echo "❌ Erreur lors de la configuration des permissions"
    exit 1
fi
