CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Status enums
CREATE TYPE file_status AS ENUM ('open', 'completed', 'aborted');
CREATE TYPE bin_status  AS ENUM ('open', 'full');

-- Users
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    innie_name      TEXT NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    password_hash   TEXT NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Sessions
CREATE TABLE sessions (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id     UUID NOT NULL REFERENCES users(id),
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ NULL
);

CREATE INDEX idx_sessions_user_id ON sessions(user_id);

-- Files (Cold Harbor)
CREATE TABLE files (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id      UUID NOT NULL REFERENCES sessions(id),
    name            TEXT NOT NULL,
    seed            BIGINT NOT NULL,
    target_per_bin  INT NOT NULL,
    status          file_status NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_files_session_id ON files(session_id);
CREATE INDEX idx_files_status ON files(status);

-- Bins (5 per file)
CREATE TABLE bins (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_id       UUID NOT NULL REFERENCES files(id),
    bin_index     INT NOT NULL,
    filled_count  INT NOT NULL DEFAULT 0,
    status        bin_status NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT unique_file_bin UNIQUE (file_id, bin_index),
    CONSTRAINT valid_bin_index CHECK (bin_index BETWEEN 0 AND 4)
);

CREATE INDEX idx_bins_file_id ON bins(file_id);
CREATE INDEX idx_bins_status ON bins(status);

-- Classifications
CREATE TABLE classifications (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bin_id      UUID NOT NULL REFERENCES bins(id),
    user_id     UUID NOT NULL REFERENCES users(id),
    numbers     JSONB NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_classifications_bin_id ON classifications(bin_id);
CREATE INDEX idx_classifications_user_id ON classifications(user_id);

-- Scores
CREATE TABLE scores (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    classification_id   UUID NOT NULL REFERENCES classifications(id),
    points              INT NOT NULL,
    tags                TEXT[] NOT NULL,
    temper_vector       JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_scores_classification_id ON scores(classification_id);

-- Events
CREATE TABLE events (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id  UUID NULL REFERENCES sessions(id),
    file_id     UUID NULL REFERENCES files(id),
    event_type  TEXT NOT NULL,
    payload     JSONB NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_events_session_id ON events(session_id);
CREATE INDEX idx_events_file_id ON events(file_id);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_created_at ON events(created_at);

-- Audit logs
CREATE TABLE audit_logs (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID NOT NULL REFERENCES users(id),
    action       TEXT NOT NULL,
    entity_type  TEXT NOT NULL,
    entity_id    UUID NOT NULL,
    payload      JSONB NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
