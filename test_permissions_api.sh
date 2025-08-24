#!/bin/bash

# Configuration
API_BASE_URL="http://localhost:3000"
EMAIL="test@example.com"
PASSWORD="password123"

echo "🔐 Test des permissions des chauffeurs"
echo "======================================"

# Test de santé
echo "1. Test de santé..."
curl -s "$API_BASE_URL/health" | jq '.'

# Test de création d'employé avec permissions
echo -e "\n2. Test de création d'employé avec permissions..."
CREATE_RESPONSE=$(curl -s -X POST "$API_BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d "{
    \"firstname\": \"Jean\",
    \"lastname\": \"Dupont\",
    \"gender\": \"M\",
    \"personal_email\": \"jean.dupont@personal.com\",
    \"login_password\": \"$PASSWORD\",
    \"phone_number\": \"+33123456789\",
    \"professional_email\": \"$EMAIL\",
    \"professional_email_password\": \"emailpass123\"
  }")

echo "Réponse création:"
echo "$CREATE_RESPONSE" | jq '.'

# Test de connexion
echo -e "\n3. Test de connexion..."
LOGIN_RESPONSE=$(curl -s -X POST "$API_BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d "{
    \"professional_email\": \"$EMAIL\",
    \"password\": \"$PASSWORD\"
  }")

echo "Réponse connexion:"
echo "$LOGIN_RESPONSE" | jq '.'

# Extraire le token JWT
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token')

if [ "$TOKEN" != "null" ] && [ "$TOKEN" != "" ]; then
    echo -e "\n4. Test d'accès aux routes des chauffeurs avec permissions..."
    
    # Test d'accès à la liste des chauffeurs (permission 23 requise)
    echo -e "\n   a) Test GET /drivers (permission 23)..."
    DRIVERS_RESPONSE=$(curl -s -X GET "$API_BASE_URL/drivers" \
      -H "Authorization: Bearer $TOKEN")
    
    echo "   Réponse:"
    echo "$DRIVERS_RESPONSE" | jq '.'
    
    # Test de création d'un chauffeur (permission 32 requise)
    echo -e "\n   b) Test POST /drivers (permission 32)..."
    CREATE_DRIVER_RESPONSE=$(curl -s -X POST "$API_BASE_URL/drivers" \
      -H "Authorization: Bearer $TOKEN" \
      -H "Content-Type: application/json" \
      -d '{
        "firstname": "Pierre",
        "lastname": "Martin",
        "license_number": "123456789",
        "phone_number": "+33123456789",
        "email": "pierre.martin@example.com"
      }')
    
    echo "   Réponse:"
    echo "$CREATE_DRIVER_RESPONSE" | jq '.'
    
    # Test d'accès sans token (doit échouer)
    echo -e "\n5. Test d'accès sans token (doit échouer)..."
    UNAUTHORIZED_RESPONSE=$(curl -s -X GET "$API_BASE_URL/drivers")
    
    echo "Réponse sans token:"
    echo "$UNAUTHORIZED_RESPONSE" | jq '.'
    
    # Test d'accès avec token invalide (doit échouer)
    echo -e "\n6. Test d'accès avec token invalide (doit échouer)..."
    INVALID_TOKEN_RESPONSE=$(curl -s -X GET "$API_BASE_URL/drivers" \
      -H "Authorization: Bearer invalid_token_here")
    
    echo "Réponse avec token invalide:"
    echo "$INVALID_TOKEN_RESPONSE" | jq '.'
    
else
    echo "❌ Échec de la connexion, impossible de tester les routes protégées"
fi

echo -e "\n✅ Tests des permissions terminés!"
echo -e "\n📋 Résumé des permissions requises:"
echo "   - GET /drivers : Permission 23 (lecture)"
echo "   - POST /drivers : Permission 32 (création)"
echo "   - GET /drivers/:id : Permission 23 (lecture)"
echo "   - PUT /drivers/:id : Permission 45 (modification)"
echo "   - DELETE /drivers/:id : Permissions 32 ET 45 (suppression)"
