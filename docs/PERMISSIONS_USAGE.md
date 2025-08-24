# Système de Permissions - Guide d'utilisation

## Vue d'ensemble

Le système de permissions permet de contrôler l'accès aux endpoints de l'API en fonction des permissions que l'employé possède dans son token JWT.

## Configuration des permissions

### 1. Définition des permissions par endpoint

Les permissions sont définies dans le fichier `src/middleware/permissions_config.rs` :

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

### 2. Identifiants d'endpoints

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

## Utilisation dans les routes

### Méthode 1 : Middleware spécialisés (recommandé)

```rust
// Créer des fonctions de middleware spécialisées
async fn drivers_list_permission_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, crate::errors::app_error::AppError> {
    let permissions = get_permissions_by_endpoint_id("drivers_list").unwrap_or_default();
    require_permissions(permissions, request, next).await
}

// Utiliser dans les routes
.route("/drivers", get(get_all_drivers))
.route_layer(axum_middleware::from_fn(drivers_list_permission_middleware))
```

### Méthode 2 : Middleware direct avec permissions

```rust
// Utiliser directement require_permissions avec une liste de permissions
.route("/drivers", get(get_all_drivers))
.route_layer(axum_middleware::from_fn(|req, next| {
    require_permissions(vec![2], req, next)
}))
```

## Ajout de nouveaux endpoints

### 1. Définir les permissions dans `permissions_config.rs`

```rust
EndpointPermissions {
    method: "POST",
    path: "/employees",
    required_permissions: &[10, 15], // Permissions 10 et 15 requises
    description: "Créer un nouvel employé",
},
```

### 2. Ajouter l'identifiant dans `get_permissions_by_endpoint_id`

```rust
"employees_create" => Some(vec![10, 15]),
```

### 3. Créer le middleware spécialisé

```rust
async fn employees_create_permission_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, crate::errors::app_error::AppError> {
    let permissions = get_permissions_by_endpoint_id("employees_create").unwrap_or_default();
    require_permissions(permissions, request, next).await
}
```

### 4. Appliquer dans les routes

```rust
.route("/employees", post(create_employee))
.route_layer(axum_middleware::from_fn(employees_create_permission_middleware))
```

## Vérification des permissions

Le système vérifie que l'employé possède **TOUTES** les permissions requises pour un endpoint.

### Exemple de vérification

- **Permissions requises** : `[10, 15]`
- **Permissions de l'employé** : `[10, 15, 20]` ✅ **Autorisé**
- **Permissions de l'employé** : `[10, 20]` ❌ **Refusé** (manque la permission 15)
- **Permissions de l'employé** : `[15, 20]` ❌ **Refusé** (manque la permission 10)

## Gestion des erreurs

Si l'employé n'a pas les permissions requises, une erreur est retournée :

```json
{
  "error": "Permissions insuffisantes. Permissions requises: [2], Permissions actuelles: [10, 20]"
}
```

## Bonnes pratiques

1. **Centraliser la configuration** : Toutes les permissions sont définies dans `permissions_config.rs`
2. **Utiliser des identifiants** : Les identifiants d'endpoints rendent le code plus lisible
3. **Documenter les permissions** : Chaque endpoint a une description claire
4. **Tester les permissions** : Vérifier que les bonnes permissions sont appliquées

## Exemple complet

```rust
// 1. Configuration des permissions
EndpointPermissions {
    method: "GET",
    path: "/reports",
    required_permissions: &[5, 8],
    description: "Accéder aux rapports",
},

// 2. Identifiant
"reports_access" => Some(vec![5, 8]),

// 3. Middleware
async fn reports_access_permission_middleware(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, crate::errors::app_error::AppError> {
    let permissions = get_permissions_by_endpoint_id("reports_access").unwrap_or_default();
    require_permissions(permissions, request, next).await
}

// 4. Application dans les routes
.route("/reports", get(get_reports))
.route_layer(axum_middleware::from_fn(reports_access_permission_middleware))
```
