# Gestion des Migrations SQLx

Ce projet utilise SQLx pour gérer l'évolution de la base de données de manière versionnée et reproductible.

## Prérequis

### Installation de SQLx CLI

```bash
# Installation de sqlx-cli avec support PostgreSQL
cargo install sqlx-cli --no-default-features --features postgres
```

### Vérification de l'installation

```bash
sqlx --version
```

## Structure des Migrations

Les migrations sont stockées dans le dossier `migrations/` et suivent le format :
- `YYYYMMDD_HHMMSS_description.sql`

Chaque fichier de migration contient :
- **Up** : Les commandes SQL à exécuter pour appliquer la migration
- **Down** : Les commandes SQL pour annuler la migration

## Utilisation

### 1. Créer une nouvelle migration

```bash
# Créer une migration pour ajouter une nouvelle table
./scripts/migrate.sh create add_user_profiles

# Créer une migration pour modifier une table existante
./scripts/migrate.sh create add_user_avatar_column
```

### 2. Exécuter les migrations

```bash
# Exécuter toutes les migrations en attente
./scripts/migrate.sh up
```

### 3. Vérifier le statut

```bash
# Afficher le statut des migrations
./scripts/migrate.sh info
```

### 4. Annuler une migration

```bash
# Annuler la dernière migration
./scripts/migrate.sh down
```

### 5. Réinitialiser la base de données

```bash
# ⚠️ ATTENTION : Supprime toutes les données !
./scripts/migrate.sh reset
```

## Commandes SQLx Directes

Vous pouvez aussi utiliser directement les commandes SQLx :

```bash
# Créer une migration
sqlx migrate add <nom_migration> --source migrations

# Exécuter les migrations
sqlx migrate run --source migrations --database-url <DATABASE_URL>

# Vérifier le statut
sqlx migrate info --source migrations --database-url <DATABASE_URL>

# Annuler la dernière migration
sqlx migrate revert --source migrations --database-url <DATABASE_URL>
```

## Intégration Docker

Les migrations sont automatiquement exécutées lors du démarrage de l'application via Docker Compose :

```bash
docker-compose up -d
```

Le service `db-migrate` s'exécute automatiquement après que PostgreSQL soit prêt.

## Bonnes Pratiques

### 1. Nommage des Migrations

- Utilisez des noms descriptifs et en minuscules
- Séparez les mots par des underscores
- Exemples : `add_user_table`, `modify_email_constraint`, `add_indexes`

### 2. Contenu des Migrations

- **Up** : Doit être idempotent (peut être exécuté plusieurs fois sans effet)
- **Down** : Doit annuler complètement les changements de la section Up
- Utilisez `IF NOT EXISTS` et `IF EXISTS` quand c'est possible

### 3. Ordre des Opérations

- Créez d'abord les tables
- Ajoutez ensuite les contraintes et index
- Insérez les données de référence en dernier

### 4. Tests

- Testez toujours vos migrations en local avant de les commiter
- Vérifiez que le rollback fonctionne correctement

## Exemple de Migration

```sql
-- Migration: add_user_profiles
-- Up
CREATE TABLE IF NOT EXISTS user_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    bio TEXT,
    avatar_url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON user_profiles(user_id);

-- Down
DROP TABLE IF EXISTS user_profiles;
```

## Dépannage

### Erreur de connexion

```bash
# Vérifier que PostgreSQL est démarré
docker-compose ps postgres

# Vérifier la connexion
./scripts/migrate.sh info
```

### Migration en échec

```bash
# Vérifier le statut
./scripts/migrate.sh info

# Annuler la dernière migration si nécessaire
./scripts/migrate.sh down

# Corriger le fichier de migration et relancer
./scripts/migrate.sh up
```

### Conflit de versions

Si vous avez des conflits de versions de migrations :

```bash
# Réinitialiser complètement la base
./scripts/migrate.sh reset

# Ou manuellement
sqlx database drop --database-url <DATABASE_URL> --yes
sqlx database create --database-url <DATABASE_URL>
sqlx migrate run --source migrations --database-url <DATABASE_URL>
```

## Intégration CI/CD

Pour l'intégration continue, ajoutez ces étapes :

```yaml
# Exemple GitHub Actions
- name: Run database migrations
  run: |
    sqlx migrate run --source migrations --database-url ${{ secrets.DATABASE_URL }}
```

## Ressources

- [Documentation SQLx](https://docs.rs/sqlx/)
- [Guide des migrations SQLx](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [Bonnes pratiques PostgreSQL](https://www.postgresql.org/docs/current/ddl.html)
