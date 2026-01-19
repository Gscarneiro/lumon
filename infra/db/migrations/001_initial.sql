CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Status enums
CREATE TYPE bin_status AS ENUM ('open', 'full');

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

CREATE UNIQUE INDEX uniq_active_session ON sessions(user_id) WHERE ended_at IS NULL;

--Files
CREATE TABLE files (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            TEXT NOT NULL UNIQUE,
    seed            BIGINT NOT NULL,
    target_per_bin  INT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

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
    user_id     UUID NOT NULL REFERENCES users(id),
    session_id  UUID NOT NULL REFERENCES sessions(id),
    file_id     UUID NOT NULL REFERENCES files(id),
    bin_id      UUID NOT NULL REFERENCES bins(id),
    numbers     JSONB NOT NULL,
    score       INT NOT NULL,
    tags        TEXT[] NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_classifications_user_id ON classifications(user_id);
CREATE INDEX idx_classifications_session_id ON classifications(session_id);
CREATE INDEX idx_classifications_file_id ON classifications(file_id);
CREATE INDEX idx_classifications_bin_id ON classifications(bin_id);

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
