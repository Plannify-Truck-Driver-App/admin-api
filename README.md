# Plannify Admin API

Une API REST moderne construite avec Rust et Axum pour la gestion des utilisateurs, entièrement conteneurisée avec Docker.

## 🚀 Démarrage rapide

### Prérequis
- Docker et Docker Compose installés
- Git

### 1. Cloner le projet
```bash
git clone <repository-url>
cd plannify-admin-api
```

### 2. Démarrer l'API complète
```bash
# Démarrer tous les services (PostgreSQL + API)
docker-compose up -d

# Ou démarrer uniquement la base de données
docker-compose up -d postgres
```

### 3. Initialiser la base de données
```bash
# Initialiser la base de données (se lance automatiquement)
docker-compose up db-init
```

### 4. Démarrer l'API
```bash
# Mode production
docker-compose up api

# Mode développement avec rechargement automatique
docker-compose up dev

# Mode test
docker-compose up test
```

## 📚 Services disponibles

| Service | Port | Description | Commande |
|---------|------|-------------|----------|
| `postgres` | 5432 | Base de données PostgreSQL | `docker-compose up postgres` |
| `db-init` | - | Initialisation de la base de données | `docker-compose up db-init` |
| `api` | 3000 | API Rust en mode production | `docker-compose up api` |
| `dev` | 3000 | API Rust avec rechargement automatique | `docker-compose up dev` |
| `test` | - | Exécution des tests | `docker-compose up test` |

## 🔧 Commandes utiles

### Gestion des services
```bash
# Démarrer tous les services
docker-compose up -d

# Démarrer un service spécifique
docker-compose up -d postgres

# Arrêter tous les services
docker-compose down

# Voir les logs
docker-compose logs -f api
docker-compose logs -f postgres

# Voir le statut
docker-compose ps
```

### Base de données
```bash
# Initialiser la base de données
docker-compose up db-init

# Se connecter à la base de données
docker-compose exec postgres psql -U plannify_user -d plannify_db

# Réinitialiser complètement
docker-compose down
docker volume rm plannify-admin-api_postgres_data
docker-compose up -d postgres
docker-compose up db-init
```

### Développement
```bash
# Mode développement avec rechargement automatique
docker-compose up dev

# Exécuter les tests
docker-compose up test

# Construire l'image
docker-compose build
```

## 📊 Endpoints de l'API

Une fois l'API démarrée, elle sera accessible sur `http://localhost:3000` :

| Méthode | Endpoint | Description |
|---------|----------|-------------|
| `GET` | `/health` | Vérification de l'état de l'API |
| `GET` | `/users` | Récupérer tous les utilisateurs |
| `POST` | `/users` | Créer un nouvel utilisateur |
| `GET` | `/users/:id` | Récupérer un utilisateur par ID |
| `PUT` | `/users/:id` | Mettre à jour un utilisateur |
| `DELETE` | `/users/:id` | Supprimer un utilisateur (soft delete) |

## 🧪 Tests

```bash
# Exécuter tous les tests
docker-compose up test

# Tests avec logs détaillés
docker-compose run --rm test cargo test -- --nocapture
```

## 🔍 Logs et monitoring

```bash
# Logs de l'API
docker-compose logs -f api

# Logs de PostgreSQL
docker-compose logs -f postgres

# Logs de tous les services
docker-compose logs -f
```

## 🚀 Déploiement

### Build de production
```bash
# Construire l'image
docker-compose build api

# Démarrer en production
docker-compose up -d api
```

### Variables d'environnement
Les variables d'environnement sont configurées dans le `docker-compose.yml` :

- `DATABASE_URL` : Connexion à PostgreSQL
- `RUST_LOG` : Niveau de logging (info, debug, trace)

## 🐛 Dépannage

### Problèmes courants

1. **Port déjà utilisé**
   ```bash
   # Vérifier les ports utilisés
   lsof -i :3000
   lsof -i :5432
   ```

2. **Base de données non accessible**
   ```bash
   # Vérifier le statut
   docker-compose ps
   
   # Voir les logs
   docker-compose logs postgres
   ```

3. **Erreur de compilation**
   ```bash
   # Reconstruire l'image
   docker-compose build --no-cache
   ```

### Réinitialisation complète
```bash
# Arrêter et nettoyer tout
docker-compose down -v
docker system prune -f

# Redémarrer depuis zéro
docker-compose up -d postgres
docker-compose up db-init
docker-compose up api
```

## 📁 Structure du projet

```
plannify-admin-api/
├── src/                    # Code source Rust
├── database/              # Scripts SQL
├── docker-compose.yml     # Configuration Docker
├── Dockerfile             # Image Docker
└── README.md              # Ce fichier
```

## 🤝 Contribution

1. Fork le projet
2. Créer une branche feature
3. Tester avec Docker Compose
4. Soumettre une pull request

## 📄 Licence

Ce projet est sous licence MIT.
