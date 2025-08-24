# API d'Authentification - Guide d'utilisation

## Vue d'ensemble

L'API d'authentification permet aux employés de se connecter et d'accéder aux ressources protégées via des tokens JWT. Chaque employé possède des permissions spécifiques basées sur son niveau d'accréditation.

## Configuration

### Variables d'environnement

Créez un fichier `.env` à la racine du projet avec les variables suivantes :

```bash
DATABASE_URL=postgresql://username:password@localhost:5432/plannify_admin
JWT_SECRET=your-super-secret-jwt-key-change-in-production
```

**⚠️ Important :** En production, utilisez une clé JWT_SECRET forte et unique !

## Endpoints

### 1. Création d'un compte employé

**POST** `/auth/register`

Crée un nouveau compte employé dans le système.

**Corps de la requête :**
```json
{
  "firstname": "Jean",
  "lastname": "Dupont",
  "gender": "M",
  "personal_email": "jean.dupont@personal.com",
  "login_password": "motdepasse123",
  "phone_number": "+33123456789",
  "professional_email": "jean.dupont@company.com",
  "professional_email_password": "emailpass123"
}
```

**Réponse :**
```json
{
  "pk_employee_id": "uuid-here",
  "firstname": "Jean",
  "lastname": "Dupont",
  "gender": "M",
  "personal_email": "jean.dupont@personal.com",
  "phone_number": "+33123456789",
  "professional_email": "jean.dupont@company.com",
  "professional_email_password": "emailpass123",
  "created_at": "2024-01-01T00:00:00Z",
  "last_login_at": null,
  "deactivated_at": null
}
```

### 2. Connexion employé

**POST** `/auth/login`

Authentifie un employé et retourne un token JWT.

**Corps de la requête :**
```json
{
  "professional_email": "jean.dupont@company.com",
  "password": "motdepasse123"
}
```

**Réponse :**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "employee": {
    "id": "uuid-here",
    "firstname": "Jean",
    "lastname": "Dupont",
    "professional_email": "jean.dupont@company.com",
    "permissions": [1, 2, 3]
  }
}
```

### 3. Vérification de santé

**GET** `/health`

Vérifie que l'API est opérationnelle.

**Réponse :**
```
200 OK
```

## Utilisation des tokens JWT

### Format du token

Le token JWT contient les informations suivantes :
- **sub** : ID de l'employé
- **email** : Email professionnel
- **firstname** : Prénom
- **lastname** : Nom
- **permissions** : Liste des IDs des permissions
- **exp** : Timestamp d'expiration
- **iat** : Timestamp de création

### Durée de vie

- **Expiration** : 24 heures
- **Renouvellement** : Nouvelle connexion requise

### Utilisation dans les requêtes

Pour accéder aux routes protégées, incluez le token dans le header `Authorization` :

```bash
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     http://localhost:3000/drivers
```

## Routes protégées

### Routes nécessitant une authentification

- `GET /drivers` - Liste des chauffeurs
- `POST /drivers` - Création d'un chauffeur
- `GET /drivers/:id` - Détails d'un chauffeur
- `PUT /drivers/:id` - Modification d'un chauffeur
- `DELETE /drivers/:id` - Suppression d'un chauffeur

### Routes avec authentification optionnelle

- `GET /drivers/public` - Liste publique des chauffeurs

## Gestion des erreurs

### Codes d'erreur courants

- **400 Bad Request** : Données de validation invalides
- **401 Unauthorized** : Token manquant ou invalide
- **403 Forbidden** : Permissions insuffisantes
- **409 Conflict** : Conflit (ex: email déjà utilisé)
- **500 Internal Server Error** : Erreur serveur

### Exemple d'erreur

```json
{
  "error": "Email ou mot de passe incorrect",
  "status": 400
}
```

## Sécurité

### Bonnes pratiques

1. **Mots de passe** : Minimum 8 caractères
2. **HTTPS** : Utilisez HTTPS en production
3. **JWT_SECRET** : Clé forte et unique
4. **Expiration** : Tokens avec durée de vie limitée
5. **Validation** : Validation côté serveur de toutes les entrées

### Hachage des mots de passe

Les mots de passe sont hachés avec bcrypt (coût par défaut) avant stockage en base.

## Exemples d'utilisation

### Script de test

Utilisez le script `test_auth_api.sh` pour tester l'API :

```bash
chmod +x test_auth_api.sh
./test_auth_api.sh
```

### Avec cURL

```bash
# Créer un compte
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"firstname":"Test","lastname":"User","professional_email":"test@example.com","login_password":"password123","personal_email":"test@personal.com","professional_email_password":"emailpass"}'

# Se connecter
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"professional_email":"test@example.com","password":"password123"}'

# Utiliser le token
TOKEN="your-jwt-token-here"
curl -H "Authorization: Bearer $TOKEN" \
     http://localhost:3000/drivers
```

## Support

Pour toute question ou problème, consultez les logs de l'application ou contactez l'équipe de développement.
