# Utilisation des filtres d'utilisateurs

## Endpoint

`GET /users`

## Paramètres de requête

### Paramètres obligatoires
- `page` (u32, défaut: 1) : Numéro de la page
- `limit` (u32, défaut: 20) : Nombre d'utilisateurs par page (max: 100)

### Paramètres d'ordre
- `sort_order` (string, défaut: "asc") : Ordre de tri par date d'ajout
  - `"asc"` : Du plus ancien au plus récent (ordre chronologique)
  - `"desc"` : Du plus récent au plus ancien (ordre anti-chronologique)

### Paramètres de filtrage optionnels

#### Filtres de texte (recherche partielle insensible à la casse)
- `firstname` (string, optionnel) : Recherche dans le prénom (utilise ILIKE avec %terme%)
- `lastname` (string, optionnel) : Recherche dans le nom (utilise ILIKE avec %terme%)
- `email` (string, optionnel) : Recherche dans l'email (utilise ILIKE avec %terme%)
- `phone_number` (string, optionnel) : Recherche dans le numéro de téléphone (utilise ILIKE avec %terme%)

#### Filtres exacts
- `pk_user_id` (string, optionnel) : ID utilisateur exact (UUID)
- `gender` (string, optionnel) : Genre exact ('M', 'F', 'O')
- `language` (string, optionnel) : Code langue exact (ex: 'fr', 'en')

#### Filtres booléens
- `is_searchable` (boolean, optionnel) : Utilisateurs recherchables ou non
- `allow_request_professional_agreement` (boolean, optionnel) : Autorisation d'accord professionnel

#### Filtres de présence de données
- `rest_json` (boolean, optionnel) : 
  - `true` : Utilisateurs ayant des données JSON personnalisées
  - `false` : Utilisateurs sans données JSON personnalisées
- `verified` (boolean, optionnel) :
  - `true` : Utilisateurs vérifiés
  - `false` : Utilisateurs non vérifiés
- `deactivated` (boolean, optionnel) :
  - `true` : Utilisateurs désactivés
  - `false` : Utilisateurs actifs

## Fonctionnalités

- **Recherche partielle** : Les champs `firstname`, `lastname`, `email` et `phone_number` utilisent `ILIKE` pour une recherche insensible à la casse
- **Filtres combinés** : Vous pouvez combiner plusieurs filtres pour affiner votre recherche
- **Filtres de présence** : Les champs `rest_json`, `verified_at` et `deactivated_at` permettent de filtrer selon la présence ou l'absence de données
- **Pagination** : Tous les filtres fonctionnent avec la pagination

## Exemples d'utilisation

### Récupérer tous les utilisateurs (première page)
```
GET /users?page=1&limit=20
```

### Rechercher des utilisateurs par prénom
```
GET /users?page=1&limit=20&firstname=john
```

### Filtrer par genre et langue
```
GET /users?page=1&limit=20&gender=M&language=fr
```

### Utilisateurs vérifiés et recherchables
```
GET /users?page=1&limit=20&verified=true&is_searchable=true
```

### Utilisateurs avec des données JSON personnalisées
```
GET /users?page=1&limit=20&rest_json=true
```

### Recherche combinée : prénom + email + vérifié
```
GET /users?page=1&limit=20&firstname=marie&email=gmail&verified=true
```

### Utilisateurs actifs (non désactivés) parlant français
```
GET /users?page=1&limit=20&language=fr&deactivated=false
```

### Récupérer TOUS les utilisateurs (y compris désactivés)
```
GET /users?page=1&limit=20
```

### Récupérer SEULEMENT les utilisateurs actifs
```
GET /users?page=1&limit=20&deactivated=false
```

### Tri par ordre chronologique (plus anciens en premier) - DÉFAUT
```
GET /users?page=1&limit=20&sort_order=asc
```

### Tri par ordre anti-chronologique (plus récents en premier)
```
GET /users?page=1&limit=20&sort_order=desc
```

### Utilisateurs actifs triés du plus récent au plus ancien
```
GET /users?page=1&limit=20&deactivated=false&sort_order=desc
```

## Réponse

La réponse suit le format de pagination standard :

```json
{
  "data": [
    {
      "pk_user_id": "uuid",
      "firstname": "John",
      "lastname": "Doe",
      "email": "john.doe@example.com",
      "phone_number": "0123456789",
      "is_searchable": true,
      "allow_request_professional_agreement": false,
      "language": "fr",
      "rest_json": null,
      "verified_at": "2024-01-15T10:30:00Z",
      "deactivated_at": null,
      // ... autres champs
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150
  }
}
```

## Notes techniques

- Tous les filtres sont optionnels et peuvent être combinés
- La recherche textuelle utilise l'opérateur `ILIKE` avec le pattern `%terme%` pour une correspondance partielle
- Les filtres booléens de présence (`rest_json`, `verified`, `deactivated`) ne comptent pas dans la numérotation des paramètres SQL
- Les résultats sont triés par date de création selon le paramètre `sort_order` (défaut: ordre chronologique ASC)
- **Sans filtres** : Aucune clause WHERE n'est appliquée, tous les utilisateurs sont retournés (y compris les désactivés)
- **Avec filtres** : Seuls les filtres spécifiés sont appliqués
- Les requêtes sont optimisées pour éviter les injections SQL en utilisant des paramètres préparés
