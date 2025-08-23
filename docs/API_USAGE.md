# Guide d'utilisation de l'API Plannify

## 🚀 Démarrage rapide

### 1. Configuration de l'environnement
```bash
# Copier le fichier d'exemple
cp env.example .env

# Éditer avec vos paramètres de base de données
DATABASE_URL=postgresql://username:password@localhost:5432/plannify_db
RUST_LOG=info
```

### 2. Initialisation de la base de données
```bash
# Se connecter à PostgreSQL
psql -U username -d plannify_db

# Exécuter le script d'initialisation
\i database/init.sql
```

### 3. Démarrage de l'API
```bash
# Utiliser le script de développement
./scripts/dev.sh

# Ou manuellement
cargo run
```

## 📚 Endpoints de l'API

### Vérification de l'état
```bash
curl http://localhost:3000/health
```

**Réponse :**
```json
200 OK
```

### Gestion des utilisateurs

#### 1. Créer un utilisateur
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "firstname": "Jean",
    "lastname": "Dupont",
    "email": "jean.dupont@example.com",
    "password": "motdepasse123",
    "language": "fr",
    "mail_preferences": 1
  }'
```

**Réponse :**
```json
{
  "pk_user_id": "550e8400-e29b-41d4-a716-446655440000",
  "firstname": "Jean",
  "lastname": "Dupont",
  "gender": null,
  "email": "jean.dupont@example.com",
  "password": "motdepasse123",
  "phone_number": null,
  "is_searchable": true,
  "allow_request_professional_agreement": true,
  "language": "fr",
  "rest_json": null,
  "mail_preferences": 1,
  "created_at": "2024-01-15T10:30:00Z",
  "verified_at": null,
  "last_login_at": null,
  "deactivated_at": null
}
```

#### 2. Récupérer tous les utilisateurs (avec pagination)
```bash
# Récupérer la première page avec 20 utilisateurs (défaut)
curl http://localhost:3000/users

# Récupérer la page 2 avec 10 utilisateurs
curl "http://localhost:3000/users?page=2&limit=10"

# Récupérer la page 1 avec 50 utilisateurs
curl "http://localhost:3000/users?page=1&limit=50"
```

**Paramètres de pagination :**
- `page` : Numéro de page (défaut : 1, minimum : 1)
- `limit` : Nombre d'utilisateurs par page (défaut : 20, maximum : 100)

**Réponse :**
```json
{
  "data": [
    {
      "pk_user_id": "550e8400-e29b-41d4-a716-446655440000",
      "firstname": "Jean",
      "lastname": "Dupont",
      "email": "jean.dupont@example.com",
      "language": "fr",
      "created_at": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "total_pages": 8,
    "has_next": true,
    "has_prev": false
  }
}
```

#### 3. Récupérer un utilisateur par ID
```bash
curl http://localhost:3000/users/550e8400-e29b-41d4-a716-446655440000
```

#### 4. Mettre à jour un utilisateur
```bash
curl -X PUT http://localhost:3000/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{
    "firstname": "Jean-Pierre",
    "phone_number": "+33123456789",
    "is_searchable": false
  }'
```

#### 5. Supprimer un utilisateur (soft delete)
```bash
curl -X DELETE http://localhost:3000/users/550e8400-e29b-41d4-a716-446655440000
```

**Réponse :**
```json
204 No Content
```

## 🔧 Tests avec différents outils

### Utilisation avec Postman
1. Importer la collection d'exemples
2. Configurer l'URL de base : `http://localhost:3000`
3. Tester les différents endpoints

### Utilisation avec Insomnia
1. Créer un nouveau projet
2. Ajouter les requêtes pour chaque endpoint
3. Utiliser les variables d'environnement

### Utilisation avec curl (exemples avancés)

#### Créer un utilisateur avec tous les champs
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "firstname": "Marie",
    "lastname": "Martin",
    "gender": "F",
    "email": "marie.martin@example.com",
    "password": "password123",
    "phone_number": "+33987654321",
    "is_searchable": true,
    "allow_request_professional_agreement": false,
    "language": "en",
    "rest_json": {
      "role": "user",
      "preferences": {
        "theme": "dark",
        "notifications": true
      }
    },
    "mail_preferences": 3
  }'
```

#### Mettre à jour plusieurs champs
```bash
curl -X PUT http://localhost:3000/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{
    "firstname": "Jean-Pierre",
    "lastname": "Dupont-Martin",
    "email": "jean-pierre.dupont@example.com",
    "phone_number": "+33123456789",
    "language": "en",
    "is_searchable": false,
    "rest_json": {
      "role": "premium",
      "subscription": "monthly"
    }
  }'
```

## 🐛 Gestion des erreurs

### Erreur 400 - Données invalides
```json
{
  "error": "Données invalides",
  "status": 400
}
```

### Erreur 400 - Paramètres de pagination invalides
```json
{
  "error": "Page must be greater than 0",
  "status": 400
}
```

```json
{
  "error": "Limit must be between 1 and 100",
  "status": 400
}
```

### Erreur 404 - Utilisateur non trouvé
```json
{
  "error": "Ressource non trouvée",
  "status": 404
}
```

### Erreur 409 - Conflit (email déjà utilisé)
```json
{
  "error": "Un utilisateur avec cet email existe déjà",
  "status": 409
}
```

### Erreur 500 - Erreur interne
```json
{
  "error": "Erreur interne du serveur",
  "status": 500
}
```

## 🔍 Logs et débogage

### Niveaux de log
- `RUST_LOG=error` - Erreurs uniquement
- `RUST_LOG=warn` - Avertissements et erreurs
- `RUST_LOG=info` - Informations, avertissements et erreurs (défaut)
- `RUST_LOG=debug` - Tous les logs
- `RUST_LOG=trace` - Logs très détaillés

### Exemple de configuration
```bash
# Dans .env
RUST_LOG=debug
RUST_BACKTRACE=1
```

## 🧪 Tests automatisés

### Exécuter les tests
```bash
# Tous les tests
cargo test

# Tests avec sortie détaillée
cargo test -- --nocapture

# Tests spécifiques
cargo test test_create_user
```

### Tests d'intégration
```bash
# Tests avec base de données de test
DATABASE_URL=postgresql://test_user:test_pass@localhost:5432/plannify_test cargo test
```

## 🚀 Déploiement

### Build de production
```bash
cargo build --release
```

### Avec Docker
```bash
# Construire l'image
docker build -t plannify-admin-api .

# Démarrer le conteneur
docker run -p 3000:3000 \
  -e DATABASE_URL=postgresql://user:pass@host:5432/db \
  plannify-admin-api
```

### Avec Docker Compose
```bash
# Démarrer tous les services
docker-compose up -d

# Voir les logs
docker-compose logs -f api
```

## 📊 Monitoring et métriques

### Endpoint de santé
```bash
curl http://localhost:3000/health
```

### Logs structurés
L'API génère des logs JSON structurés pour faciliter l'analyse :
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "info",
  "message": "Création d'un nouvel utilisateur",
  "email": "user@example.com",
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

## 🔐 Sécurité

### Bonnes pratiques
1. **Toujours utiliser HTTPS en production**
2. **Valider toutes les entrées utilisateur**
3. **Hasher les mots de passe** (bcrypt utilisé)
4. **Limiter le taux de requêtes** (rate limiting)
5. **Utiliser des tokens JWT** pour l'authentification

### Validation des données
- Emails : format valide et unicité
- Mots de passe : longueur minimale
- Numéros de téléphone : format international
- Langues : codes ISO 639-1 valides

## 🤝 Support et contribution

### Signaler un bug
1. Vérifier les logs de l'API
2. Reproduire le problème
3. Créer une issue sur GitHub avec :
   - Description du problème
   - Étapes de reproduction
   - Logs d'erreur
   - Version de l'API

### Proposer une amélioration
1. Créer une issue pour discuter
2. Fork le projet
3. Créer une branche feature
4. Soumettre une pull request

### Questions et aide
- Consulter la documentation Rust et Axum
- Vérifier les exemples dans ce guide
- Ouvrir une issue sur GitHub
