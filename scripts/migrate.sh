#!/bin/bash

# Script de gestion des migrations SQLx
# Usage: ./scripts/migrate.sh [create|up|down|info|reset]

set -e

# Configuration
DATABASE_URL="postgresql://plannify_user:plannify_password@localhost:5432/plannify_db"
MIGRATIONS_DIR="migrations"

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction d'aide
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commandes disponibles:"
    echo "  create <name>  Créer une nouvelle migration"
    echo "  up             Exécuter toutes les migrations en attente"
    echo "  down           Annuler la dernière migration"
    echo "  info           Afficher le statut des migrations"
    echo "  reset          Réinitialiser la base de données (supprime toutes les données !)"
    echo "  help           Afficher cette aide"
    echo ""
    echo "Exemples:"
    echo "  $0 create add_user_profile"
    echo "  $0 up"
    echo "  $0 info"
}

# Vérifier que sqlx-cli est installé
check_sqlx() {
    if ! command -v sqlx &> /dev/null; then
        echo -e "${RED}❌ sqlx-cli n'est pas installé${NC}"
        echo "Installez-le avec: cargo install sqlx-cli --no-default-features --features postgres"
        exit 1
    fi
}

# Vérifier la connexion à la base de données
check_db_connection() {
    echo -e "${BLUE}🔍 Vérification de la connexion à la base de données...${NC}"
    
    if ! sqlx database ping --database-url "$DATABASE_URL" &> /dev/null; then
        echo -e "${RED}❌ Impossible de se connecter à la base de données${NC}"
        echo "Vérifiez que PostgreSQL est démarré et accessible"
        echo "URL: $DATABASE_URL"
        exit 1
    fi
    
    echo -e "${GREEN}✅ Connexion à la base de données établie${NC}"
}

# Créer une nouvelle migration
create_migration() {
    local name="$1"
    
    if [ -z "$name" ]; then
        echo -e "${RED}❌ Nom de migration requis${NC}"
        echo "Usage: $0 create <name>"
        exit 1
    fi
    
    echo -e "${BLUE}📝 Création de la migration: $name${NC}"
    
    # Créer la migration avec sqlx
    sqlx migrate add "$name" --source "$MIGRATIONS_DIR"
    
    echo -e "${GREEN}✅ Migration créée avec succès${NC}"
    echo "Fichier créé dans: $MIGRATIONS_DIR/"
}

# Exécuter les migrations
run_migrations() {
    echo -e "${BLUE}🚀 Exécution des migrations...${NC}"
    
    sqlx migrate run --source "$MIGRATIONS_DIR" --database-url "$DATABASE_URL"
    
    echo -e "${GREEN}✅ Migrations exécutées avec succès${NC}"
}

# Annuler la dernière migration
rollback_migration() {
    echo -e "${YELLOW}⚠️  Annulation de la dernière migration...${NC}"
    
    sqlx migrate revert --source "$MIGRATIONS_DIR" --database-url "$DATABASE_URL"
    
    echo -e "${GREEN}✅ Dernière migration annulée${NC}"
}

# Afficher le statut des migrations
show_migration_status() {
    echo -e "${BLUE}📊 Statut des migrations:${NC}"
    
    sqlx migrate info --source "$MIGRATIONS_DIR" --database-url "$DATABASE_URL"
}

# Réinitialiser la base de données
reset_database() {
    echo -e "${RED}⚠️  ATTENTION: Cette action va supprimer toutes les données !${NC}"
    echo -e "${YELLOW}Êtes-vous sûr de vouloir continuer ? (y/N)${NC}"
    read -r response
    
    if [[ "$response" =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}🗑️  Suppression de la base de données...${NC}"
        
        sqlx database drop --database-url "$DATABASE_URL" --yes
        
        echo -e "${BLUE}🔄 Recréation de la base de données...${NC}"
        sqlx database create --database-url "$DATABASE_URL"
        
        echo -e "${BLUE}🚀 Exécution des migrations...${NC}"
        sqlx migrate run --source "$MIGRATIONS_DIR" --database-url "$DATABASE_URL"
        
        echo -e "${GREEN}✅ Base de données réinitialisée avec succès${NC}"
    else
        echo -e "${YELLOW}❌ Opération annulée${NC}"
    fi
}

# Script principal
main() {
    local command="$1"
    
    case "$command" in
        "create")
            create_migration "$2"
            ;;
        "up")
            check_sqlx
            check_db_connection
            run_migrations
            ;;
        "down")
            check_sqlx
            check_db_connection
            rollback_migration
            ;;
        "info")
            check_sqlx
            check_db_connection
            show_migration_status
            ;;
        "reset")
            check_sqlx
            check_db_connection
            reset_database
            ;;
        "help"|"--help"|"-h"|"")
            show_help
            ;;
        *)
            echo -e "${RED}❌ Commande inconnue: $command${NC}"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Exécuter le script principal
main "$@"
