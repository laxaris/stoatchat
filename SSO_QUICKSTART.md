# Stoatchat Keycloak SSO - Quick Start

Bu dosya, Keycloak SSO entegrasyonunu hızlıca test etmek için gerekli adımları içerir.

## Hızlı Başlangıç

```bash
# 1. SSO servislerini başlat
./scripts/setup-sso.sh

# 2. Keycloak'ı konfigüre et (tarayıcıda)
# http://localhost:8080 - admin/admin ile giriş yap
# Realm: stoatchat
# Client: stoatchat-backend

# 3. Client secret'ı .env.sso dosyasına ekle

# 4. OAuth2 Proxy'yi yeniden başlat
docker-compose -f docker-compose.sso.yml restart oauth2-proxy

# 5. Stoatchat'i çalıştır
cargo run --bin revolt-delta

# 6. Test et
# Tarayıcıda: http://localhost:4180/users/@me
```

## Detaylı Dokümantasyon

Tam kurulum rehberi için: [docs/KEYCLOAK_SETUP.md](docs/KEYCLOAK_SETUP.md)

## Dosyalar

- `docker-compose.sso.yml` - Keycloak, OAuth2 Proxy, Nginx servisleri
- `nginx-sso.conf` - Nginx reverse proxy konfigürasyonu
- `.env.sso.example` - Environment variables örneği
- `scripts/setup-sso.sh` - Otomatik kurulum script'i
- `crates/core/database/src/models/users/axum.rs` - SSO authentication middleware
- `crates/core/database/src/models/users/ops/reference.rs` - User provisioning implementasyonu

## Mimari

```
Browser → Nginx (OAuth2 Proxy) → Keycloak → Stoatchat API
                ↓
        X-Auth-Email header
        X-Auth-User header
                ↓
        Otomatik user provisioning
```

## Sorun Giderme

```bash
# Logları kontrol et
docker-compose -f docker-compose.sso.yml logs -f

# Servisleri yeniden başlat
docker-compose -f docker-compose.sso.yml restart

# Servisleri durdur
docker-compose -f docker-compose.sso.yml down
```
