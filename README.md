# Astra Nest — IVF Journey Companion

## Structure
- `backend/` — Rust (Axum) API. Deploy on Railway. Talks to Gemini (interpretation) and AWS S3 (report files).
- `mobile/` — Flutter app (Android first, iOS later from the same codebase).

## Backend setup (local)
1. Install Rust: https://rustup.rs
2. `cd backend`
3. Copy `.env.example` to `.env` and fill in real values (Gemini API key, AWS keys, a Postgres URL, a JWT secret).
4. `cargo run` — starts on port 8080, runs migrations automatically on boot.

## Deploying to Railway
1. Push this repo to GitHub.
2. In Railway: New Project → Deploy from GitHub repo → pick `backend/` as the root.
3. Add a Postgres plugin in the same Railway project — it auto-injects `DATABASE_URL`.
4. Add the other env vars from `.env.example` in Railway's Variables tab (GEMINI_API_KEY, AWS_*, S3_BUCKET, JWT_SECRET).
5. Railway builds with Nixpacks automatically detecting Rust — first build takes a few minutes.
6. Once deployed, copy the public URL into `mobile/lib/services/api_service.dart` (`baseUrl`).

## AWS S3 setup
1. Create an S3 bucket (e.g. `astra-nest-reports`) in a region close to your users (ap-south-1 for India).
2. Create an IAM user with a policy scoped to just `s3:PutObject`/`s3:GetObject` on that bucket — never use root credentials.
3. Put the access key/secret into Railway's env vars.

## Mobile setup (local)
1. Install Flutter: https://docs.flutter.dev/get-started/install
2. `cd mobile && flutter pub get`
3. `flutter run` (with an Android emulator or device connected)
4. Before building a release APK, update `baseUrl` in `lib/services/api_service.dart` to your live Railway URL.

## What's scaffolded vs. what's next
Done: auth (signup/login), report file upload to S3, single-marker Gemini interpretation call,
TWW symptom logging with a curated reassurance map, doctor-referral-code patient lookup, DB schema.

Still to build: OCR/marker extraction from the uploaded file (currently just stores the raw file —
next step is a Gemini vision call or AWS Textract call to pull out marker name/value pairs automatically),
trends chart wiring, chat screen wired to the interpret endpoint, doctor dashboard UI, push notifications.

## Safety note
The system prompt in `backend/src/gemini.rs` (`SYSTEM_GUARDRAIL`) is the core liability control for
this app — it keeps every AI response non-diagnostic and defers to the treating doctor. Don't loosen it
without thinking through the regulatory angle first.
