# Configuration des Tokens JWT

## Variables d'environnement

### ACCESS_TOKEN_DURATION_HOURS
- **Description** : Durée de validité de l'access token en heures
- **Valeur par défaut** : 1 heure
- **Exemple** : `ACCESS_TOKEN_DURATION_HOURS=1`

### REFRESH_TOKEN_DURATION_HOURS
- **Description** : Durée de validité du refresh token en heures
- **Valeur par défaut** : 168 heures (7 jours)
- **Exemple** : `REFRESH_TOKEN_DURATION_HOURS=168`

### JWT_SECRET
- **Description** : Clé secrète pour signer les tokens JWT
- **Valeur par défaut** : "your-secret-key-change-in-production"
- **Exemple** : `JWT_SECRET=your-super-secret-key-here`

## Exemple de fichier .env

```bash
# Configuration de la base de données
DATABASE_URL=postgresql://plannify_user:plannify_password@localhost:5432/plannify_admin

# Clé secrète JWT (changez en production !)
JWT_SECRET=your-secret-key-change-in-production

# Durée de validité des tokens (en heures)
ACCESS_TOKEN_DURATION_HOURS=1
REFRESH_TOKEN_DURATION_HOURS=168
```

## Utilisation

### 1. Connexion
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"professional_email": "user@example.com", "password": "password123"}'
```

**Réponse** :
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "employee": {
    "id": "uuid-here",
    "firstname": "John",
    "lastname": "Doe",
    "professional_email": "user@example.com",
    "permissions": [1, 2, 3]
  }
}
```

### 2. Refresh du token
```bash
curl -X POST http://localhost:3000/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{"refresh_token": "your-refresh-token-here"}'
```

**Réponse** :
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

### 3. Utilisation de l'access token
```bash
curl -H "Authorization: Bearer your-access-token-here" \
  http://localhost:3000/drivers
```

## Sécurité

- **Access Token** : Court terme (1 heure par défaut), contient les permissions
- **Refresh Token** : Long terme (7 jours par défaut), contient seulement l'ID de l'employé
- **Rotation** : Les refresh tokens peuvent être révoqués en changeant la clé secrète
- **HTTPS** : Utilisez toujours HTTPS en production

## Bonnes pratiques

1. **Stockage sécurisé** : Stockez le refresh token de manière sécurisée (httpOnly cookie)
2. **Rotation des clés** : Changez régulièrement la clé secrète JWT
3. **Logs** : Surveillez les tentatives de refresh pour détecter les abus
4. **Expiration** : Ajustez les durées selon vos besoins de sécurité
