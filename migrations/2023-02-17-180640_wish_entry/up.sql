CREATE TABLE IF NOT EXISTS wishes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    comment TEXT NOT NULL,
    score SMALLINT NOT NULL DEFAULT 1,
    sessionid UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS votes (
    wishid SERIAL REFERENCES wishes(id),
    sessionid UUID REFERENCES sessions(id),
    PRIMARY KEY (wishid, sessionid)
);