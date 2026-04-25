CREATE TABLE IF NOT EXISTS device_models (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL UNIQUE,
    width           INTEGER NOT NULL,
    height          INTEGER NOT NULL,
    is_virtual      BOOLEAN NOT NULL DEFAULT 0,
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Seed initial device profiles
INSERT INTO device_models (name, width, height, is_virtual) VALUES
    ('TRMNL OG 7.5"', 800, 480, 0),
    ('TRMNL X 10.3"', 1872, 1404, 0),
    ('Adafruit MagTag 2.9"', 296, 128, 0),
    ('Virtual (800×480)', 800, 480, 1);
