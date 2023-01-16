CREATE TABLE futhark (
    id SERIAL PRIMARY KEY,
    rune_set VARCHAR(255) NOT NULL
);

CREATE TABLE rune (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    latin VARCHAR(255) NOT NULL,
    futhark INT NOT NULL,
    CONSTRAINT futhark_id FOREIGN KEY (futhark) REFERENCES futhark(id)
);

