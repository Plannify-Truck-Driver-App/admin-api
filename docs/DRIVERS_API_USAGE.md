# Utilisation de l'API des Drivers

## Endpoint

`GET /drivers`

## Paramètres de requête

### Paramètres obligatoires
- `page` (u32, défaut: 1) : Numéro de la page
- `limit` (u32, défaut: 20) : Nombre de drivers par page (max: 100)

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
- `pk_driver_id` (string, optionnel) : ID driver exact (UUID)
- `gender` (string, optionnel) : Genre du driver
  - `"M"` : Drivers masculins
  - `"F"` : Drivers féminins  
  - `"O"` : Drivers avec autre genre
  - `"none"` : **Drivers sans genre défini (gender IS NULL)**
- `language` (string, optionnel) : Code langue exact (ex: 'fr', 'en')

#### Filtres booléens
- `is_searchable` (boolean, optionnel) : Drivers recherchables ou non
- `allow_request_professional_agreement` (boolean, optionnel) : Autorisation d'accord professionnel

#### Filtres de présence de données
- `rest_json` (boolean, optionnel) : 
  - `true` : Drivers ayant des données JSON personnalisées
  - `false` : Drivers sans données JSON personnalisées
- `verified` (boolean, optionnel) :
  - `true` : Drivers vérifiés
  - `false` : Drivers non vérifiés
- `deactivated` (boolean, optionnel) :
  - `true` : Drivers désactivés
  - `false` : Drivers actifs

## Fonctionnalités

- **Recherche partielle** : Les champs `firstname`, `lastname`, `email` et `phone_number` utilisent `ILIKE` pour une recherche insensible à la casse
- **Filtres combinés** : Vous pouvez combiner plusieurs filtres pour affiner votre recherche
- **Filtres de présence** : Les champs `rest_json`, `verified` et `deactivated` permettent de filtrer selon la présence ou l'absence de données
- **Filtre spécial gender=none** : Permet de trouver les drivers qui n'ont pas de genre défini
- **Pagination** : Tous les filtres fonctionnent avec la pagination

## Exemples d'utilisation

### Récupérer tous les drivers (première page)
```
GET /drivers?page=1&limit=20
```

### Rechercher des drivers par prénom
```
GET /drivers?page=1&limit=20&firstname=john
```

### Filtrer par genre et langue
```
GET /drivers?page=1&limit=20&gender=M&language=fr
```

### Drivers vérifiés et recherchables
```
GET /drivers?page=1&limit=20&verified=true&is_searchable=true
```

### Drivers avec des données JSON personnalisées
```
GET /drivers?page=1&limit=20&rest_json=true
```

### **NOUVEAU : Drivers sans genre défini**
```
GET /drivers?page=1&limit=20&gender=none
```

### Recherche combinée : prénom + email + vérifié
```
GET /drivers?page=1&limit=20&firstname=marie&email=gmail&verified=true
```

### Drivers actifs (non désactivés) parlant français
```
GET /drivers?page=1&limit=20&language=fr&deactivated=false
```

### **NOUVEAU : Drivers actifs sans genre défini**
```
GET /drivers?page=1&limit=20&gender=none&deactivated=false
```

### Tri par ordre chronologique (plus anciens en premier) - DÉFAUT
```
GET /drivers?page=1&limit=20&sort_order=asc
```

### Tri par ordre anti-chronologique (plus récents en premier)
```
GET /drivers?page=1&limit=20&sort_order=desc
```

### Drivers sans genre triés du plus récent au plus ancien
```
GET /drivers?page=1&limit=20&gender=none&sort_order=desc
```

## Réponse

La réponse suit le format de pagination standard :

```json
{
  "data": [
    {
      "pk_driver_id": "uuid",
      "firstname": "John",
      "lastname": "Doe",
      "gender": null,
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

## Cas d'usage du filtre gender=none

Le paramètre `gender=none` est particulièrement utile pour :

1. **Gestion des données incomplètes** : Identifier les drivers dont le profil n'est pas complètement rempli
2. **Conformité RGPD** : Respecter les préférences des drivers qui ne souhaitent pas divulguer leur genre
3. **Analyse des données** : Comprendre combien de drivers n'ont pas encore complété cette information
4. **Campagnes marketing** : Adapter les communications selon les préférences des drivers

## Notes techniques

- Tous les filtres sont optionnels et peuvent être combinés
- La recherche textuelle utilise l'opérateur `ILIKE` avec le pattern `%terme%` pour une correspondance partielle
- Les filtres booléens de présence (`rest_json`, `verified`, `deactivated`) ne comptent pas dans la numérotation des paramètres SQL
- Les résultats sont triés par date de création selon le paramètre `sort_order` (défaut: ordre chronologique ASC)
- **Sans filtres** : Aucune clause WHERE n'est appliquée, tous les drivers sont retournés (y compris les désactivés)
- **Avec filtres** : Seuls les filtres spécifiés sont appliqués
- **Filtre spécial gender=none** : Génère la clause SQL `gender IS NULL` sans paramètre binding
- Les requêtes sont optimisées pour éviter les injections SQL en utilisant des paramètres préparés
