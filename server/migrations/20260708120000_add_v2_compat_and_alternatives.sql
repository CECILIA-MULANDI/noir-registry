-- v2 additions: compat matrix, maintenance metadata, alternatives, comparison notes.
-- All changes are additive. No columns dropped, no existing rows touched.
-- See research/schema_v2.md for design rationale.

-- Maintenance + notes on packages
ALTER TABLE packages
    ADD COLUMN IF NOT EXISTS last_commit_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS comparison_notes TEXT;

CREATE INDEX IF NOT EXISTS idx_packages_last_commit_at ON packages(last_commit_at DESC);

-- Nargo compatibility matrix: one row per (package, nargo_version).
-- Populated nightly by the compat-runner binary via `nargo check`.
CREATE TABLE IF NOT EXISTS package_compat_results (
    package_id      INTEGER      NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    nargo_version   TEXT         NOT NULL,
    checked_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    status          TEXT         NOT NULL CHECK (status IN ('ok', 'failed', 'error')),
    error_snippet   TEXT,
    PRIMARY KEY (package_id, nargo_version)
);

CREATE INDEX IF NOT EXISTS idx_compat_results_nargo_version ON package_compat_results(nargo_version);
CREATE INDEX IF NOT EXISTS idx_compat_results_status ON package_compat_results(status);

-- Alternatives: symmetric edges between packages that solve the same problem.
-- Two rows per pair (a → b, b → a) so lookup is one-directional.
CREATE TABLE IF NOT EXISTS package_alternatives (
    package_id      INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    alternative_id  INTEGER NOT NULL REFERENCES packages(id) ON DELETE CASCADE,
    PRIMARY KEY (package_id, alternative_id),
    CHECK (package_id <> alternative_id)
);

CREATE INDEX IF NOT EXISTS idx_alternatives_package ON package_alternatives(package_id);
CREATE INDEX IF NOT EXISTS idx_alternatives_alternative ON package_alternatives(alternative_id);
