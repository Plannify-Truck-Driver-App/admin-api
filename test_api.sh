#!/bin/bash

echo "🧪 Test de l'API Plannify Admin"
echo "================================="

# URL de base (ajustez selon votre configuration)
BASE_URL="http://localhost:3000"

echo ""
echo "1️⃣ Test de récupération des utilisateurs (page 1, limit 100)"
echo "GET $BASE_URL/users?page=1&limit=100"
echo ""

# Test de la requête
curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=100" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "2️⃣ Test de récupération des utilisateurs (page 1, limit 20)"
echo "GET $BASE_URL/users?page=1&limit=20"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=20" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "3️⃣ Test avec filtre sur le prénom"
echo "GET $BASE_URL/users?page=1&limit=20&firstname=john"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=20&firstname=john" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "4️⃣ Test avec filtre pour utilisateurs actifs seulement"
echo "GET $BASE_URL/users?page=1&limit=20&deactivated=false"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=20&deactivated=false" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "5️⃣ Test tri chronologique (plus anciens en premier) - DÉFAUT"
echo "GET $BASE_URL/users?page=1&limit=10&sort_order=asc"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=10&sort_order=asc" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "6️⃣ Test tri anti-chronologique (plus récents en premier)"
echo "GET $BASE_URL/users?page=1&limit=10&sort_order=desc"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/users?page=1&limit=10&sort_order=desc" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "✅ Tests terminés"
echo ""
echo "💡 Si vous obtenez des erreurs 500, vérifiez :"
echo "   - Que votre base de données est accessible"
echo "   - Que les migrations ont été appliquées"
echo "   - Les logs de votre application"
echo ""
echo "ℹ️  Notes importantes :"
echo "   - Sans filtres : TOUS les utilisateurs sont retournés (y compris désactivés)"
echo "   - Avec deactivated=false : Seulement les utilisateurs actifs"
echo "   - sort_order=asc : Plus anciens en premier (DÉFAUT)"
echo "   - sort_order=desc : Plus récents en premier"
echo ""
echo "🔍 Pour déboguer, lancez votre application avec :"
echo "   RUST_LOG=debug cargo run"
