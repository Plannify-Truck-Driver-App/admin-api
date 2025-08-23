#!/bin/bash

echo "🚗 Test de l'API Plannify Drivers"
echo "=================================="

# URL de base (ajustez selon votre configuration)
BASE_URL="http://localhost:3000"

echo ""
echo "1️⃣ Test de récupération de tous les drivers (page 1, limit 100)"
echo "GET $BASE_URL/drivers?page=1&limit=100"
echo ""

# Test de la requête
curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=100" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "2️⃣ Test de récupération des drivers (page 1, limit 20)"
echo "GET $BASE_URL/drivers?page=1&limit=20"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "3️⃣ Test avec filtre sur le prénom"
echo "GET $BASE_URL/drivers?page=1&limit=20&firstname=john"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&firstname=john" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "4️⃣ Test avec filtre pour drivers actifs seulement"
echo "GET $BASE_URL/drivers?page=1&limit=20&deactivated=false"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&deactivated=false" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "5️⃣ Test avec filtre gender=none (drivers sans genre)"
echo "GET $BASE_URL/drivers?page=1&limit=20&gender=none"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&gender=none" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "6️⃣ Test avec filtre gender=M (drivers masculins)"
echo "GET $BASE_URL/drivers?page=1&limit=20&gender=M"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&gender=M" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "7️⃣ Test avec filtre gender=F (drivers féminins)"
echo "GET $BASE_URL/drivers?page=1&limit=20&gender=F"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&gender=F" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "8️⃣ Test combiné : drivers sans genre ET actifs"
echo "GET $BASE_URL/drivers?page=1&limit=20&gender=none&deactivated=false"
echo ""

curl -s -w "\n\nStatus: %{http_code}\nTemps: %{time_total}s\n" \
     "$BASE_URL/drivers?page=1&limit=20&gender=none&deactivated=false" | jq '.' 2>/dev/null || echo "Erreur: Impossible de parser la réponse JSON"

echo ""
echo "✅ Tests terminés"
echo ""
echo "💡 Si vous obtenez des erreurs 500, vérifiez :"
echo "   - Que votre base de données est accessible"
echo "   - Que les migrations ont été appliquées"
echo "   - Les logs de votre application"
echo ""
echo "ℹ️  Notes importantes :"
echo "   - Sans filtres : TOUS les drivers sont retournés (y compris désactivés)"
echo "   - Avec deactivated=false : Seulement les drivers actifs"
echo "   - gender=none : Drivers sans genre défini (gender IS NULL)"
echo "   - gender=M/F : Drivers avec genre spécifique"
echo "   - sort_order=asc : Plus anciens en premier (DÉFAUT)"
echo "   - sort_order=desc : Plus récents en premier"
echo ""
echo "🔍 Pour déboguer, lancez votre application avec :"
echo "   RUST_LOG=debug cargo run"
