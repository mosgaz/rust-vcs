# rust-vcs

Корпоративный сервис коммуникаций на Rust по ТЗ из `docs/SPECIFICATION.md`.

Технологии UI: **Leptos (SSR)** + **Tailwind CSS**.

## Что реализовано

### Этап 1: MVP

- Авторизация сотрудников: регистрация + логин c JWT токеном.
- Главная UI-страница на Leptos (`GET /`) со стилями Tailwind.
- Создание встречи и выдача ссылки-приглашения.
- Вход внешнего участника по ссылке (без регистрации).
- Текстовый чат встречи через WebSocket (модель DataChannel на этапе MVP).
- WebRTC signaling канал для P2P-видеосессий: `GET /v1/meetings/:slug/signal/ws`.
- Отправка файлов в чате (payload в base64).
- Личные сообщения (direct messages) между сотрудниками.
- Базовая мультиязычность ответа (`en`/`ru`) через `Accept-Language`.
- Подготовка к TLS 1.3: отдельный модуль конфигурации TLS.

### Этап 2: Основной функционал (текущее состояние)

Реализовано в рамках текущей итерации:

- **Корпоративный мессенджер (персистентные треды в памяти процесса):**
  - создание тредов/каналов;
  - получение списка тредов;
  - отправка сообщений в тред;
  - получение сообщений треда.
- **Режим вебинара:**
  - создание встречи с `mode=webinar`;
  - назначение спикеров организатором (`/webinar/speakers`).
- **Запись встреч (server-side placeholder):**
  - запуск сессии записи;
  - сохранение метаданных записи в in-memory store;
  - генерация синтетического `storage_uri`.
- **Desktop/Tauri readiness:**
  - endpoint со статусом целевых платформ (windows/macos/linux).
- **UI-страницы Этапа 2 (SSR-шаблоны):**
  - `/auth/login` — страница авторизации;
  - `/auth/register` — страница регистрации;
  - `/messenger` — страница мессенджера;
  - `/meetings/:slug` — страница встречи;
  - `/meetings/:slug/waiting-room` — страница комнаты ожидания.

> Важно: на текущем шаге данные Этапа 2 хранятся **in-memory** (без PostgreSQL),
> а запись встреч реализована как серверный каркас (без полноценного медиа-рекордера).

## Локальный запуск

```bash
cargo run
```

По умолчанию сервис стартует на `http://localhost:8080`.

Если порт занят, укажите другой:

```bash
RUST_VCS_PORT=18080 cargo run
# или
PORT=18080 cargo run
```

## Основные UI маршруты

- `GET /`
- `GET /auth/login`
- `GET /auth/register`
- `GET /messenger`
- `GET /meetings/:slug`
- `GET /meetings/:slug/waiting-room`

## Основные API эндпоинты

- `GET /health`
- `POST /v1/auth/register`
- `POST /v1/auth/login`
- `POST /v1/meetings`
- `POST /v1/meetings/:slug/join`
- `GET /v1/meetings/:slug/ws`
- `GET /v1/meetings/:slug/signal/ws`
- `POST /v1/messages/direct`
- `POST /v1/messenger/threads`
- `GET /v1/messenger/threads`
- `POST /v1/messenger/threads/:thread_id/messages`
- `GET /v1/messenger/threads/:thread_id/messages`
- `POST /v1/meetings/:slug/webinar/speakers`
- `POST /v1/meetings/:slug/recordings/start`
- `GET /v1/desktop/status`

## Примечание по медиа/DTLS/SFU

Сейчас реализован signaling и каркас серверных сценариев Этапа 2. Полноценный SFU-режим на Mediasoup, production-ready рекордер медиапотоков и постоянное хранилище (PostgreSQL/S3) остаются в следующих итерациях roadmap.
