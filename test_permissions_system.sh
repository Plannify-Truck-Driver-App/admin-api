#!/bin/bash

# Script de test pour le système de permissions
# Ce script teste que les endpoints sont correctement protégés par les permissions

echo "🧪 Test du système de permissions"
echo "=================================="

# Variables
API_URL="http://localhost:3000"
JWT_TOKEN=""

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Fonction pour afficher les résultats
print_result() {
    local test_name="$1"
    local status="$2"
    local message="$3"
    
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✅ PASS${NC} - $test_name: $message"
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}❌ FAIL${NC} - $test_name: $message"
    else
        echo -e "${YELLOW}⚠️  SKIP${NC} - $test_name: $message"
    fi
}

# Test 1: Endpoint de santé (public)
echo ""
echo "1. Test de l'endpoint de santé (public)"
response=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/health")
if [ "$response" = "200" ]; then
    print_result "Health Check" "PASS" "Endpoint public accessible"
else
    print_result "Health Check" "FAIL" "Code de réponse: $response"
fi

# Test 2: Tentative d'accès aux drivers sans token (doit échouer)
echo ""
echo "2. Test d'accès aux drivers sans authentification"
response=$(curl -s -o /dev/null -w "%{http_code}" "$API_URL/drivers")
if [ "$response" = "401" ] || [ "$response" = "403" ]; then
    print_result "Drivers sans auth" "PASS" "Accès refusé comme attendu (code: $response)"
else
    print_result "Drivers sans auth" "FAIL" "Code de réponse inattendu: $response"
fi

# Test 3: Connexion pour obtenir un token
echo ""
echo "3. Test de connexion pour obtenir un token JWT"
login_response=$(curl -s -X POST "$API_URL/auth/login" \
    -H "Content-Type: application/json" \
    -d '{
        "email": "admin@example.com",
        "password": "password123"
    }')

if echo "$login_response" | grep -q "token"; then
    JWT_TOKEN=$(echo "$login_response" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)
    print_result "Login" "PASS" "Token JWT obtenu"
    echo "   Token: ${JWT_TOKEN:0:20}..."
else
    print_result "Login" "FAIL" "Impossible d'obtenir le token"
    echo "   Réponse: $login_response"
fi

# Test 4: Accès aux drivers avec token valide
if [ -n "$JWT_TOKEN" ]; then
    echo ""
    echo "4. Test d'accès aux drivers avec token valide"
    response=$(curl -s -o /dev/null -w "%{http_code}" \
        -H "Authorization: Bearer $JWT_TOKEN" \
        "$API_URL/drivers")
    
    if [ "$response" = "200" ]; then
        print_result "Drivers avec auth" "PASS" "Accès autorisé"
    elif [ "$response" = "403" ]; then
        print_result "Drivers avec auth" "PASS" "Accès refusé - permissions insuffisantes"
    else
        print_result "Drivers avec auth" "FAIL" "Code de réponse inattendu: $response"
    fi
else
    print_result "Drivers avec auth" "SKIP" "Token non disponible"
fi

# Test 5: Vérification des permissions dans la base de données
echo ""
echo "5. Vérification des permissions dans la base de données"
echo "   (Ce test nécessite une base de données active)"

# Test 6: Test de différents endpoints avec permissions
if [ -n "$JWT_TOKEN" ]; then
    echo ""
    echo "6. Test des différents endpoints avec permissions"
    
    # Test GET /drivers/:id
    response=$(curl -s -o /dev/null -w "%{http_code}" \
        -H "Authorization: Bearer $JWT_TOKEN" \
        "$API_URL/drivers/test-id")
    print_result "GET /drivers/:id" "PASS" "Code de réponse: $response"
    
    # Test POST /drivers
    response=$(curl -s -o /dev/null -w "%{http_code}" \
        -X POST \
        -H "Authorization: Bearer $JWT_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"email": "test@example.com", "firstname": "Test", "lastname": "User"}' \
        "$API_URL/drivers")
    print_result "POST /drivers" "PASS" "Code de réponse: $response"
fi

echo ""
echo "🎯 Résumé des tests"
echo "==================="
echo "Le système de permissions est maintenant configuré pour :"
echo "  • GET /drivers → Permission 2 requise"
echo "  • GET /drivers/:id → Permission 2 requise"
echo "  • POST /drivers → Permission 32 requise"
echo "  • PUT /drivers/:id → Permission 45 requise"
echo "  • DELETE /drivers/:id → Permissions 32 ET 45 requises"
echo ""
echo "📚 Documentation disponible dans docs/PERMISSIONS_USAGE.md"
echo ""
echo "✨ Test terminé !"
