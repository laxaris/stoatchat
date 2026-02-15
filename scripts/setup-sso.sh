#!/bin/bash

# Keycloak SSO Quick Start Script
# Bu script Keycloak ve OAuth2 Proxy'yi başlatır

set -e

echo "🚀 Stoatchat Keycloak SSO Setup"
echo "================================"

# Check if .env.sso exists
if [ ! -f .env.sso ]; then
    echo "⚠️  .env.sso dosyası bulunamadı. Örnek dosyadan oluşturuluyor..."
    cp .env.sso.example .env.sso
    
    # Generate cookie secret
    COOKIE_SECRET=$(openssl rand -base64 32 | tr -d '\n')
    
    # Update .env.sso with generated secret
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/changeme1234567890123456789012/$COOKIE_SECRET/" .env.sso
    else
        sed -i "s/changeme1234567890123456789012/$COOKIE_SECRET/" .env.sso
    fi
    
    echo "✅ .env.sso oluşturuldu"
    echo "⚠️  OAUTH2_CLIENT_SECRET değerini Keycloak'tan alıp .env.sso dosyasına eklemeyi unutmayın!"
fi

# Load environment variables
export $(cat .env.sso | grep -v '^#' | xargs)

echo ""
echo "📦 Docker servisleri başlatılıyor..."
docker-compose -f docker-compose.sso.yml up -d

echo ""
echo "⏳ Keycloak'ın hazır olması bekleniyor..."
sleep 10

# Wait for Keycloak to be ready
until curl -s http://localhost:8080/health/ready > /dev/null 2>&1; do
    echo "   Keycloak henüz hazır değil, bekleniyor..."
    sleep 5
done

echo ""
echo "✅ Keycloak hazır!"
echo ""
echo "📋 Sonraki Adımlar:"
echo "   1. Keycloak Admin Console: http://localhost:8080"
echo "      Username: admin"
echo "      Password: admin"
echo ""
echo "   2. Realm oluştur: 'stoatchat'"
echo ""
echo "   3. Client oluştur: 'stoatchat-backend'"
echo "      - Client authentication: ON"
echo "      - Valid redirect URIs: http://localhost:4180/oauth2/callback"
echo ""
echo "   4. Client Secret'ı kopyala ve .env.sso dosyasına ekle"
echo ""
echo "   5. OAuth2 Proxy'yi yeniden başlat:"
echo "      docker-compose -f docker-compose.sso.yml restart oauth2-proxy"
echo ""
echo "   6. Test kullanıcısı oluştur ve test et"
echo ""
echo "📖 Detaylı kurulum için: docs/KEYCLOAK_SETUP.md"
echo ""
echo "🔗 Servisler:"
echo "   - Keycloak:     http://localhost:8080"
echo "   - OAuth2 Proxy: http://localhost:4180"
echo "   - Nginx Gateway: http://localhost:8000"
echo ""
