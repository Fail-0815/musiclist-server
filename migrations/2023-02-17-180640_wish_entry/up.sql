-- Your SQL goes here
CREATE TABLE wishes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    artist VARCHAR NOT NULL,
    comment TEXT NOT NULL,
    score SMALLINT NOT NULL DEFAULT 1,
    sessionid UUID NOT NULL
);

CREATE TABLE sessions (
    id UUID PRIMARY KEY
);

CREATE TABLE votes (
    wishid SERIAL REFERENCES wishes(id),
    sessionid UUID REFERENCES sessions(id),
    PRIMARY KEY (wishid, sessionid)
);

