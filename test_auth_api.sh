#!/bin/bash

# Configuration
API_BASE_URL="http://localhost:3000"
EMAIL="test@example.com"
PASSWORD="password123"

echo "🧪 Test de l'API d'authentification"
echo "=================================="

# Test de santé
echo "1. Test de santé..."
curl -s "$API_BASE_URL/health" | jq '.'

# Test de création d'employé
echo -e "\n2. Test de création d'employé..."
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
    echo -e "\n4. Test d'accès protégé avec token JWT..."
    
    # Test d'accès à une route protégée
    PROTECTED_RESPONSE=$(curl -s -X GET "$API_BASE_URL/drivers" \
      -H "Authorization: Bearer $TOKEN")
    
    echo "Réponse route protégée:"
    echo "$PROTECTED_RESPONSE" | jq '.'
    
    echo -e "\n5. Test d'accès sans token (doit échouer)..."
    UNAUTHORIZED_RESPONSE=$(curl -s -X GET "$API_BASE_URL/drivers")
    
    echo "Réponse sans token:"
    echo "$UNAUTHORIZED_RESPONSE" | jq '.'
    
else
    echo "❌ Échec de la connexion, impossible de tester les routes protégées"
fi

echo -e "\n✅ Tests terminés!"
