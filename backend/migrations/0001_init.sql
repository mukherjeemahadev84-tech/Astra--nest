CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    phone TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    doctor_referral_code TEXT,
    cycle_stage TEXT DEFAULT 'not_started',
    trial_ends_at TIMESTAMPTZ NOT NULL DEFAULT (now() + interval '30 days'),
    is_pro BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    cycle_day INT,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS report_markers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_id UUID REFERENCES reports(id),
    marker_name TEXT NOT NULL,
    value TEXT NOT NULL,
    unit TEXT,
    flagged BOOLEAN DEFAULT false
);

CREATE TABLE IF NOT EXISTS tww_logs (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    symptom TEXT NOT NULL,
    note TEXT,
    logged_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS doctors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    clinic_name TEXT,
    referral_code TEXT UNIQUE NOT NULL,
    trial_ends_at TIMESTAMPTZ NOT NULL DEFAULT (now() + interval '30 days'),
    is_paying BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
