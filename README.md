# Plannify Admin API

API d'administration pour la plateforme Plannify, permettant la gestion des employés, des chauffeurs et des autorisations.

## 🚀 Fonctionnalités

- **Authentification JWT** : Système de connexion sécurisé pour les employés
- **Gestion des employés** : Création et authentification des comptes employés
- **Gestion des chauffeurs** : CRUD complet pour les chauffeurs
- **Système de permissions** : Gestion fine des autorisations par niveau d'employé
- **API REST** : Interface HTTP complète avec validation des données

## 🏗️ Architecture

- **Backend** : Rust avec Axum
- **Base de données** : PostgreSQL avec SQLx
- **Authentification** : JWT avec bcrypt pour le hachage des mots de passe
- **Validation** : Validation des données avec le crate `validator`
- **Conteneurisation** : Docker et Docker Compose

## 📋 Prérequis

- Rust 1.70+
- Docker et Docker Compose
- PostgreSQL 15+

## 🛠️ Installation

### 1. Cloner le projet

```bash
git clone <repository-url>
cd plannify-admin/api
```

### 2. Configuration des variables d'environnement

Créez un fichier `.env` à la racine du projet :

```bash
DATABASE_URL=postgresql://username:password@localhost:5432/plannify_admin
JWT_SECRET=your-super-secret-jwt-key-change-in-production
```

### 3. Démarrage avec Docker (Recommandé)

```bash
# Démarrer l'API et PostgreSQL
./scripts/start-api.sh

# Ou manuellement
docker-compose up --build -d
```

### 4. Démarrage en local

```bash
# Installer les dépendances
cargo install sqlx-cli

# Créer la base de données
sqlx database create

# Exécuter les migrations
sqlx migrate run

# Démarrer l'API
cargo run
```

## 🗄️ Base de données

### Tables principales

- **employees** : Comptes employés avec informations personnelles et professionnelles
- **employee_levels** : Niveaux hiérarchiques des employés
- **employee_authorizations** : Permissions disponibles dans le système
- **employee_accreditation_authorizations** : Attribution des permissions aux employés
- **drivers** : Informations sur les chauffeurs

### Migrations

Les migrations sont gérées avec SQLx CLI :

```bash
# Créer une nouvelle migration
sqlx migrate add nom_de_la_migration

# Exécuter les migrations
sqlx migrate run

# Annuler la dernière migration
sqlx migrate revert
```

## 🔐 Authentification

### Création d'un compte employé

```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "firstname": "Jean",
    "lastname": "Dupont",
    "professional_email": "jean.dupont@company.com",
    "login_password": "motdepasse123",
    "personal_email": "jean.dupont@personal.com",
    "professional_email_password": "emailpass123"
  }'
```

### Connexion

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "professional_email": "jean.dupont@company.com",
    "password": "motdepasse123"
  }'
```

### Utilisation du token JWT

```bash
TOKEN="your-jwt-token-here"
curl -H "Authorization: Bearer $TOKEN" \
     http://localhost:3000/drivers
```

## 📚 Documentation API

- [Guide d'authentification](docs/AUTH_API_USAGE.md)
- [Guide des chauffeurs](docs/DRIVERS_API_USAGE.md)
- [Guide des migrations](docs/MIGRATIONS.md)

## 🧪 Tests

### Tests de l'API d'authentification

```bash
./test_auth_api.sh
```

### Tests des chauffeurs

```bash
./test_drivers_api.sh
```

## 🔧 Développement

### Structure du projet

```
src/
├── main.rs              # Point d'entrée de l'application
├── models/              # Modèles de données
│   ├── employee.rs      # Modèle employé
│   ├── driver.rs        # Modèle chauffeur
│   └── jwt.rs           # Modèles JWT
├── handlers/            # Gestionnaires des requêtes HTTP
│   ├── auth_handlers.rs # Gestionnaires d'authentification
│   └── driver_handlers.rs # Gestionnaires des chauffeurs
├── database/            # Services de base de données
│   ├── auth_service.rs  # Service d'authentification
│   └── driver_service.rs # Service des chauffeurs
├── middleware/          # Middleware HTTP
│   └── auth.rs          # Middleware d'authentification
└── errors/              # Gestion des erreurs
    └── app_error.rs     # Types d'erreurs de l'application
```

### Ajout de nouvelles fonctionnalités

1. **Modèles** : Créer les structures dans `src/models/`
2. **Services** : Implémenter la logique métier dans `src/database/`
3. **Handlers** : Créer les endpoints HTTP dans `src/handlers/`
4. **Migrations** : Ajouter les tables nécessaires
5. **Tests** : Créer des scripts de test

## 🚀 Déploiement

### Production

1. **Variables d'environnement** : Configurez `JWT_SECRET` avec une clé forte
2. **Base de données** : Utilisez une instance PostgreSQL gérée
3. **HTTPS** : Configurez un reverse proxy avec SSL/TLS
4. **Monitoring** : Ajoutez des métriques et des logs structurés

### Docker

```bash
# Build de l'image
docker build -t plannify-admin-api .

# Exécution
docker run -p 3000:3000 \
  -e DATABASE_URL=your-db-url \
  -e JWT_SECRET=your-secret \
  plannify-admin-api
```

## 🤝 Contribution

1. Fork le projet
2. Créez une branche feature (`git checkout -b feature/AmazingFeature`)
3. Committez vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrez une Pull Request

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 🆘 Support

Pour toute question ou problème :
- Consultez la documentation dans le dossier `docs/`
- Vérifiez les logs de l'application
- Ouvrez une issue sur le repository

---

**Note** : Ce projet est en développement actif. L'API peut évoluer et certaines fonctionnalités peuvent ne pas être encore implémentées.
