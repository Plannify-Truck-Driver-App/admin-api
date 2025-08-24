# Système de Permissions - Guide d'utilisation

## Vue d'ensemble

Le système de permissions de Plannify Admin API contrôle l'accès aux différentes fonctionnalités en fonction des codes de permission attribués aux employés. Chaque employé possède un ensemble de permissions basé sur son niveau d'accréditation dans le système.

## 🔐 Codes de Permission des Chauffeurs

### Permissions Requises

Pour accéder aux routes des chauffeurs, les employés doivent posséder les permissions suivantes :

| Code | Description | Routes |
|------|-------------|---------|
| **23** | **Lecture des chauffeurs** | `GET /drivers`, `GET /drivers/:id` |
| **32** | **Création des chauffeurs** | `POST /drivers` |
| **45** | **Modification des chauffeurs** | `PUT /drivers/:id` |
| **32 + 45** | **Suppression des chauffeurs** | `DELETE /drivers/:id` |

### Logique de Vérification

- **Lecture** : Permission 23 requise
- **Création** : Permission 32 requise
- **Modification** : Permission 45 requise
- **Suppression** : **TOUTES** les permissions 32 ET 45 requises

## 🏗️ Architecture des Permissions

### Structure des Tables

Le système de permissions repose sur plusieurs tables SQL :

```sql
-- Niveaux d'employés
employee_levels

-- Types d'autorisations
employee_authorization_types

-- Autorisations spécifiques
employee_authorizations

-- Liaison entre niveaux et autorisations
link_employee_authorization

-- Accréditation des employés
employee_accreditation_authorizations
```

### Flux de Vérification

1. **Authentification** : L'employé se connecte et reçoit un token JWT
2. **Extraction des permissions** : Le token contient la liste des codes de permission
3. **Vérification des permissions** : Chaque route vérifie les permissions requises
4. **Accès autorisé/refusé** : L'API autorise ou refuse l'accès selon les permissions

## 🔧 Middleware de Permissions

### Middleware Disponibles

```rust
// Vérification de toutes les permissions requises
require_permissions(vec![23, 32, 45], request, next)

// Vérification des permissions des chauffeurs
require_driver_read_permission(request, next)      // Permission 23
require_driver_create_permission(request, next)    // Permission 32
require_driver_update_permission(request, next)    // Permission 45
require_driver_delete_permissions(request, next)   // Permissions 32 ET 45

// Vérification d'au moins une permission
require_any_permission(vec![23, 32, 45], request, next)
```

### Application des Middleware

```rust
// Route protégée avec permission de lecture
.route("/drivers", get(get_all_drivers))
.route_layer(axum_middleware::from_fn(require_driver_read_permission))

// Route protégée avec permission de création
.route("/drivers", post(create_driver))
.route_layer(axum_middleware::from_fn(require_driver_create_permission))
```

## 📋 Exemples d'Utilisation

### Scénario 1 : Employé avec Permission de Lecture Seule

**Permissions** : `[23]`

**Accès autorisé** :
- ✅ `GET /drivers` - Liste des chauffeurs
- ✅ `GET /drivers/:id` - Détails d'un chauffeur

**Accès refusé** :
- ❌ `POST /drivers` - Création de chauffeur
- ❌ `PUT /drivers/:id` - Modification de chauffeur
- ❌ `DELETE /drivers/:id` - Suppression de chauffeur

### Scénario 2 : Employé avec Permissions de Création et Modification

**Permissions** : `[32, 45]`

**Accès autorisé** :
- ✅ `POST /drivers` - Création de chauffeur
- ✅ `PUT /drivers/:id` - Modification de chauffeur

**Accès refusé** :
- ❌ `GET /drivers` - Lecture des chauffeurs
- ❌ `DELETE /drivers/:id` - Suppression de chauffeur

### Scénario 3 : Employé avec Toutes les Permissions

**Permissions** : `[23, 32, 45]`

**Accès autorisé** :
- ✅ Toutes les routes des chauffeurs

## 🚨 Gestion des Erreurs

### Erreur de Permissions Insuffisantes

```json
{
  "error": "Permissions insuffisantes. Permissions requises: [23, 32, 45], Permissions actuelles: [23]",
  "status": 400
}
```

### Erreur d'Authentification

```json
{
  "error": "Authentification requise",
  "status": 400
}
```

## 🧪 Tests des Permissions

### Script de Test

Utilisez le script `test_permissions_api.sh` pour tester le système de permissions :

```bash
chmod +x test_permissions_api.sh
./test_permissions_api.sh
```

### Tests Manuels

```bash
# 1. Créer un employé
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"firstname":"Test","lastname":"User","professional_email":"test@example.com","login_password":"password123","personal_email":"test@personal.com","professional_email_password":"emailpass"}'

# 2. Se connecter
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"professional_email":"test@example.com","password":"password123"}'

# 3. Tester les permissions
TOKEN="your-jwt-token-here"

# Test de lecture (permission 23)
curl -H "Authorization: Bearer $TOKEN" \
     http://localhost:3000/drivers

# Test de création (permission 32)
curl -X POST -H "Authorization: Bearer $TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"firstname":"Pierre","lastname":"Martin","license_number":"123456789"}' \
     http://localhost:3000/drivers
```

## 🔒 Sécurité

### Bonnes Pratiques

1. **Vérification systématique** : Toutes les routes sensibles vérifient les permissions
2. **Principle of Least Privilege** : Chaque employé n'a que les permissions nécessaires
3. **Validation côté serveur** : Les permissions sont toujours vérifiées côté serveur
4. **Logs d'accès** : Tous les accès sont tracés pour audit

### Gestion des Tokens

- **Expiration** : 24 heures
- **Renouvellement** : Nouvelle connexion requise
- **Révocation** : Suppression de l'employé ou désactivation de son compte

## 📈 Extension du Système

### Ajout de Nouvelles Permissions

1. **Base de données** : Ajouter le code de permission dans `employee_authorizations`
2. **Middleware** : Créer un nouveau middleware de vérification
3. **Routes** : Appliquer le middleware aux routes appropriées
4. **Tests** : Mettre à jour les scripts de test

### Exemple d'Extension

```rust
// Nouvelle permission pour la gestion des véhicules
pub async fn require_vehicle_permissions(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    require_permissions(vec![50, 51, 52], request, next).await
}

// Application aux routes des véhicules
.route("/vehicles", get(get_all_vehicles))
.route_layer(axum_middleware::from_fn(require_vehicle_permissions))
```

## 🆘 Dépannage

### Problèmes Courants

1. **Erreur "Permissions insuffisantes"**
   - Vérifiez que l'employé a les bonnes permissions dans la base
   - Vérifiez que les permissions sont bien incluses dans le token JWT

2. **Erreur "Authentification requise"**
   - Vérifiez que le token JWT est valide et non expiré
   - Vérifiez que le header `Authorization` est correctement formaté

3. **Permissions non reconnues**
   - Vérifiez la structure des tables de permissions
   - Vérifiez les relations entre les tables

### Vérification des Permissions

```sql
-- Vérifier les permissions d'un employé
SELECT 
    e.professional_email,
    ea.feature_code,
    ea.pk_employee_authorization_id
FROM employees e
JOIN employee_accreditation_authorizations eaa ON e.pk_employee_id = eaa.fk_recipient_employee_id
JOIN employee_levels el ON eaa.fk_employee_level_id = el.pk_employee_level_id
JOIN link_employee_authorization lea ON el.pk_employee_level_id = lea.fk_employee_level_id
JOIN employee_authorization_types eat ON lea.fk_employee_authorization_type_id = eat.pk_employee_authorization_type_id
JOIN employee_authorizations ea ON eat.pk_employee_authorization_type_id = ea.pk_employee_authorization_id
WHERE e.professional_email = 'test@example.com';
```

---

**Note** : Ce système de permissions est conçu pour être flexible et extensible. Contactez l'équipe de développement pour toute question ou demande d'extension.
