# Système de Permissions - Plannify Admin API

## 🎯 Vue d'ensemble

Ce système de permissions permet de contrôler l'accès aux endpoints de l'API en fonction des permissions que l'employé possède dans son token JWT. Chaque endpoint peut exiger une ou plusieurs permissions spécifiques.

## 🏗️ Architecture

### Structure des fichiers

```
src/middleware/
├── permissions.rs          # Logique de vérification des permissions
├── permissions_config.rs   # Configuration des permissions par endpoint
└── auth.rs                 # Middleware d'authentification JWT
```

### Flux de vérification

1. **Authentification** : Le middleware `auth_middleware` vérifie le token JWT
2. **Extraction des permissions** : Les permissions sont extraites du token et stockées dans `AuthState`
3. **Vérification des permissions** : Le middleware de permissions vérifie que l'employé a les permissions requises
4. **Exécution du handler** : Si les permissions sont suffisantes, le handler est exécuté

## ⚙️ Configuration

### Définition des permissions par endpoint

Les permissions sont définies dans `src/middleware/permissions_config.rs` :

```rust
pub const ENDPOINT_PERMISSIONS: &[EndpointPermissions] = &[
    EndpointPermissions {
        method: "GET",
        path: "/drivers",
        required_permissions: &[2], // Permission 2 pour récupérer la liste des chauffeurs
        description: "Récupérer la liste des chauffeurs",
    },
    // ... autres endpoints
];
```

### Identifiants d'endpoints

Pour simplifier l'utilisation, des identifiants sont définis :

```rust
pub fn get_permissions_by_endpoint_id(endpoint_id: &str) -> Option<Vec<i32>> {
    match endpoint_id {
        "drivers_list" => Some(vec![2]),      // GET /drivers
        "drivers_read" => Some(vec![2]),      // GET /drivers/:id
        "drivers_create" => Some(vec![32]),   // POST /drivers
        "drivers_update" => Some(vec![45]),   // PUT /drivers/:id
        "drivers_delete" => Some(vec![32, 45]), // DELETE /drivers/:id
        _ => None,
    }
}
```

## 🚀 Utilisation

### Dans les routes

```rust
// Utilisation avec identifiants d'endpoints
.route("/drivers", get(get_all_drivers))
.route_layer(axum_middleware::from_fn(|req, next| {
    let permissions = get_permissions_by_endpoint_id("drivers_list").unwrap_or_default();
    require_permissions(permissions, req, next)
}))
```

### Permissions actuelles configurées

| Endpoint | Méthode | Permissions requises | Description |
|----------|---------|---------------------|-------------|
| `/drivers` | GET | `[2]` | Récupérer la liste des chauffeurs |
| `/drivers/:id` | GET | `[2]` | Récupérer un chauffeur par ID |
| `/drivers` | POST | `[32]` | Créer un nouveau chauffeur |
| `/drivers/:id` | PUT | `[45]` | Modifier un chauffeur existant |
| `/drivers/:id` | DELETE | `[32, 45]` | Supprimer un chauffeur |

## 🔐 Logique de vérification

### Règles de permissions

- **Toutes les permissions requises** : L'employé doit posséder **TOUTES** les permissions listées
- **Ordre des permissions** : L'ordre n'a pas d'importance
- **Permissions supplémentaires** : L'employé peut avoir plus de permissions que requises

### Exemples de vérification

| Permissions requises | Permissions employé | Résultat |
|---------------------|---------------------|----------|
| `[2]` | `[2, 10, 15]` | ✅ **Autorisé** |
| `[2, 32]` | `[2, 32, 45]` | ✅ **Autorisé** |
| `[2, 32]` | `[2, 45]` | ❌ **Refusé** (manque 32) |
| `[2, 32]` | `[10, 15]` | ❌ **Refusé** (manque 2 et 32) |

## 📝 Ajout de nouveaux endpoints

### 1. Définir les permissions

```rust
// Dans permissions_config.rs
EndpointPermissions {
    method: "POST",
    path: "/employees",
    required_permissions: &[10, 15],
    description: "Créer un nouvel employé",
},
```

### 2. Ajouter l'identifiant

```rust
// Dans get_permissions_by_endpoint_id
"employees_create" => Some(vec![10, 15]),
```

### 3. Appliquer dans les routes

```rust
.route("/employees", post(create_employee))
.route_layer(axum_middleware::from_fn(|req, next| {
    let permissions = get_permissions_by_endpoint_id("employees_create").unwrap_or_default();
    require_permissions(permissions, req, next)
}))
```

## 🧪 Tests

### Script de test

Un script de test est disponible : `test_permissions_system.sh`

```bash
./test_permissions_system.sh
```

### Tests manuels

```bash
# Test sans authentification (doit échouer)
curl http://localhost:3000/drivers

# Test avec authentification
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://localhost:3000/drivers
```

## 🚨 Gestion des erreurs

### Erreurs de permissions

Si l'employé n'a pas les permissions requises :

```json
{
  "error": "Permissions insuffisantes. Permissions requises: [2], Permissions actuelles: [10, 20]"
}
```

### Codes de statut HTTP

- **200** : Succès avec permissions suffisantes
- **401** : Token JWT manquant ou invalide
- **403** : Permissions insuffisantes
- **500** : Erreur interne du serveur

## 🔧 Configuration avancée

### Middleware personnalisé

Vous pouvez créer des middleware personnalisés pour des cas spécifiques :

```rust
async fn custom_permission_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, crate::errors::app_error::AppError> {
    // Logique personnalisée de vérification des permissions
    let permissions = vec![5, 10, 15];
    require_permissions(permissions, request, next).await
}
```

### Vérification conditionnelle

```rust
// Vérifier au moins une permission
let has_any_permission = required_permissions
    .iter()
    .any(|&required| auth_state.permissions.contains(&required));
```

## 📚 Documentation

- **Guide d'utilisation** : `docs/PERMISSIONS_USAGE.md`
- **API Reference** : `docs/API_USAGE.md`
- **Système d'authentification** : `docs/AUTH_API_USAGE.md`

## 🆘 Support

### Problèmes courants

1. **Erreur 403** : Vérifiez que l'employé a les bonnes permissions
2. **Erreur 401** : Vérifiez que le token JWT est valide
3. **Permissions manquantes** : Vérifiez la configuration dans `permissions_config.rs`

### Debug

Activez les logs pour déboguer :

```bash
RUST_LOG=debug cargo run
```

## 🔄 Évolution du système

### Versions futures

- [ ] Support des rôles et groupes de permissions
- [ ] Permissions dynamiques basées sur le contexte
- [ ] Audit trail des accès aux endpoints
- [ ] Interface d'administration des permissions

---

**Note** : Ce système remplace l'ancien système de permissions hardcodées et offre une approche plus flexible et maintenable.
