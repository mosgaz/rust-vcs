# rust-vcs

MVP корпоративного сервиса коммуникаций на Rust по ТЗ из `docs/SPECIFICATION.md`.

Технологии UI в MVP: **Leptos (SSR)** + **Tailwind CSS**.

## Что реализовано (Этап 1: MVP)

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

## Локальный запуск

```bash
cargo run
```

Сервис стартует на `http://localhost:8080`.

## Основные API эндпоинты

- `GET /health`
- `GET /` (Leptos + Tailwind UI)
- `POST /v1/auth/register`
- `POST /v1/auth/login`
- `POST /v1/meetings`
- `POST /v1/meetings/:slug/join`
- `GET /v1/meetings/:slug/ws`
- `GET /v1/meetings/:slug/signal/ws`
- `POST /v1/messages/direct`

## Примечание по медиа/DTLS

В этом MVP реализован signaling для P2P-сценария (что соответствует пункту «1 на 1 (P2P) или малая группа через Mediasoup SFU»). Полноценный SFU-режим на Mediasoup и DTLS на медиатранспорте остаются в следующей итерации.
