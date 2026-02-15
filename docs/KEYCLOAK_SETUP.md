# Keycloak SSO Setup Guide

Bu dokümantasyon, Stoatchat için Keycloak SSO entegrasyonunu nasıl kuracağınızı açıklar.

## Gereksinimler

- Docker ve Docker Compose
- Çalışan Stoatchat instance'ı

## Kurulum Adımları

### 1. SSO Servislerini Başlatın

```bash
# SSO servislerini başlat (Keycloak + OAuth2 Proxy)
docker-compose -f docker-compose.sso.yml up -d

# Logları kontrol et
docker-compose -f docker-compose.sso.yml logs -f
```

Servisler:
- **Keycloak**: http://localhost:8080
- **OAuth2 Proxy**: http://localhost:4180
- **Nginx (SSO Gateway)**: http://localhost:8000

### 2. Keycloak Admin Console'a Giriş

1. http://localhost:8080 adresine gidin
2. **Administration Console** tıklayın
3. Giriş yapın:
   - Username: `admin`
   - Password: `admin`

### 3. Realm Oluşturun

1. Sol üstteki realm dropdown'dan **Create Realm** seçin
2. Realm name: `stoatchat`
3. **Create** butonuna tıklayın

### 4. Client Oluşturun

1. Sol menüden **Clients** → **Create client**
2. Aşağıdaki bilgileri girin:

   **General Settings:**
   - Client type: `OpenID Connect`
   - Client ID: `stoatchat-backend`
   - Name: `Stoatchat Backend`
   - Description: `OAuth2 client for Stoatchat API`

3. **Next** butonuna tıklayın

   **Capability config:**
   - Client authentication: `ON` ✅
   - Authorization: `OFF`
   - Authentication flow:
     - Standard flow: `ON` ✅
     - Direct access grants: `ON` ✅

4. **Next** butonuna tıklayın

   **Login settings:**
   - Valid redirect URIs: `http://localhost:4180/oauth2/callback`
   - Valid post logout redirect URIs: `http://localhost:4180`
   - Web origins: `http://localhost:4180`

5. **Save** butonuna tıklayın

### 5. Client Secret'ı Alın

1. **Clients** → `stoatchat-backend` → **Credentials** tab
2. **Client secret** değerini kopyalayın
3. `.env.sso` dosyasını oluşturun:

```bash
cp .env.sso.example .env.sso
```

4. `OAUTH2_CLIENT_SECRET` değerini yapıştırın

### 6. Cookie Secret Oluşturun

```bash
# Random cookie secret oluştur
openssl rand -base64 32
```

Çıktıyı `.env.sso` dosyasındaki `OAUTH2_COOKIE_SECRET` değerine yapıştırın.

### 7. Test Kullanıcısı Oluşturun

1. Sol menüden **Users** → **Add user**
2. Kullanıcı bilgileri:
   - Username: `testuser`
   - Email: `test@example.com`
   - First name: `Test`
   - Last name: `User`
   - Email verified: `ON` ✅

3. **Create** butonuna tıklayın

4. **Credentials** tab'ına gidin
5. **Set password** butonuna tıklayın:
   - Password: `testpass123`
   - Temporary: `OFF`
   - **Save** butonuna tıklayın

### 8. OAuth2 Proxy'yi Yeniden Başlatın

```bash
# .env.sso dosyasını yükle
export $(cat .env.sso | xargs)

# OAuth2 Proxy'yi yeniden başlat
docker-compose -f docker-compose.sso.yml restart oauth2-proxy
```

### 9. Stoatchat'i SSO Modunda Çalıştırın

Stoatchat koduna SSO middleware'i ekledikten sonra:

```bash
# Stoatchat'i normal şekilde başlat
cargo run --bin revolt-delta
```

## Test Etme

### 1. OAuth2 Proxy Üzerinden Giriş

```bash
# OAuth2 Proxy'ye istek at (redirect olacak)
curl -v http://localhost:4180/users/@me
```

Tarayıcıda http://localhost:4180/users/@me adresine gidin:
1. Keycloak login sayfasına yönlendirileceksiniz
2. `testuser` / `testpass123` ile giriş yapın
3. Stoatchat API'ye authenticated istek atılacak

### 2. Nginx Gateway Üzerinden Giriş

```bash
# Nginx SSO gateway üzerinden
curl -v http://localhost:8000/users/@me
```

### 3. Kullanıcı Otomatik Oluşturma Testi

İlk giriş yaptığınızda, Stoatchat veritabanında otomatik olarak kullanıcı oluşturulacak:

```bash
# MongoDB'de kontrol et
docker exec -it stoatchat-database mongosh revolt

# Kullanıcıları listele
db.users.find({ username: "testuser" }).pretty()
```

## Production Deployment

### 1. HTTPS Kullanın

```yaml
# docker-compose.sso.yml
oauth2-proxy:
  command:
    - --cookie-secure=true  # HTTPS için
    - --redirect-url=https://yourdomain.com/oauth2/callback
```

### 2. Keycloak'ı Production Modunda Çalıştırın

```yaml
keycloak:
  command: start
  environment:
    KC_HOSTNAME: auth.yourdomain.com
    KC_PROXY: edge
```

### 3. Nginx SSL Konfigürasyonu

```nginx
server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;

    # ... rest of config
}
```

## Troubleshooting

### OAuth2 Proxy Redirect Loop

**Problem:** Sürekli Keycloak'a redirect oluyor

**Çözüm:**
1. Cookie secret'ın doğru olduğundan emin olun
2. Redirect URI'nin Keycloak client'ta tanımlı olduğunu kontrol edin
3. Browser cookies'leri temizleyin

### User Provisioning Çalışmıyor

**Problem:** Giriş yapılıyor ama Stoatchat'te kullanıcı oluşmuyor

**Çözüm:**
1. Nginx loglarını kontrol edin: `docker logs stoatchat-nginx-sso`
2. Header'ların doğru geçtiğini kontrol edin
3. Stoatchat loglarında SSO user creation mesajlarını arayın

### Keycloak'a Erişilemiyor

**Problem:** Connection refused hatası

**Çözüm:**
```bash
# Keycloak container'ının çalıştığını kontrol et
docker ps | grep keycloak

# Logları kontrol et
docker logs stoatchat-keycloak

# Database bağlantısını kontrol et
docker logs stoatchat-keycloak-db
```

## Ek Özellikler

### Social Login (Google, GitHub, etc.)

1. Keycloak Admin Console → **Identity Providers**
2. Provider seçin (Google, GitHub, Facebook, etc.)
3. Client ID ve Secret girin
4. Kullanıcılar artık social login yapabilir

### Multi-Factor Authentication (MFA)

1. Keycloak Admin Console → **Authentication**
2. **Required Actions** → **Configure OTP** ekleyin
3. Kullanıcılar ilk girişte OTP setup yapmak zorunda kalacak

### Role-Based Access Control

1. Keycloak'ta roller oluşturun
2. `sso.rs` dosyasında role mapping ekleyin
3. Stoatchat permission sistemine entegre edin

## Destek

Sorun yaşarsanız:
1. Docker logs kontrol edin
2. Keycloak event logs'a bakın (Events → Login Events)
3. OAuth2 Proxy debug mode açın: `--logging-level=debug`
