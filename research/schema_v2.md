# Noir Registry v2: Schema Design

## Why this exists

In May 2026, a Noir maintainer told me the categorization in v1 "didn't much make sense." I paused, built more Noir projects to understand the space, then went to look at **how devs actually search for libraries.**

I harvested **1,813 real dev interactions** across:

| Source                                | Rows  |
| ------------------------------------- | ----- |
| `noir-lang/noir` GitHub Discussions   | 131   |
| `noir-lang/noir` issues (most-recent) | 1,000 |
| `AztecProtocol/aztec-packages` issues | 637   |
| `noir-lang/noir-bignum` issues        | 45    |

**444 matched** on library-discovery / primitive / version-pain vocabulary. Raw CSVs live in `research/harvest/`. This document is what I learned and what I want to change.

## Two findings

**1. Devs search by primitive name, not by category.** They don't type "Cryptography"; they type "bls12-381", "elgamal", "poseidon", "ecdsa". A category tree fights this. The v1 categories (Cryptography / Data Structures / Math / Utilities / Zero Knowledge / Circuits / Standards) don't map onto how anyone searches.

**2. Nargo version compat is a chronic, first-class pain, and no registry surfaces it.** Real, recent examples from the harvest:

- [noir-array-helpers v0.30.4 fails to compile due to integer bit width mismatch on latest Noir](https://github.com/noir-lang/noir/issues/12348)
- [Tests fail on latest Nargo nightly release](https://github.com/noir-lang/noir-bignum/issues/262). This is on `noir-bignum` itself. Even the flagship library breaks on nightly.
- [Guidance on Aztec NR compile errors, suspected Noir/Nargo version mismatch](https://github.com/AztecProtocol/aztec-packages/issues/16431). A real dev asking for exactly this.
- [Circuit size blowup from Aztec v3-v4 (root cause potentially in Noir compiler)](https://github.com/noir-lang/noir/issues/12411)

## Two lenses, same data

1. **"I need a library for X"**: primitive / use-case tag search
2. **"What compiles on my Nargo version?"**: compat matrix / maintenance view

The same underlying entries surface through two different entry points.

## Schema changes

**Kept as-is:** `packages`, `package_versions`, `package_keywords`, `users`.

**Removed:** `categories`, `package_categories`. Tags replace them as the sole organizing axis for search and browse.

**New columns on `packages`:**

| Column             | Type          | Purpose                                                                |
| ------------------ | ------------- | ---------------------------------------------------------------------- |
| `last_commit_at`   | `TIMESTAMPTZ` | Fetched from GitHub `pushed_at`, feeds `maintenance_status` derivation |
| `comparison_notes` | `TEXT`        | Free-form "vs alternatives" prose                                      |

**New tables:**

- `package_compat_results (package_id, nargo_version, checked_at, status, error_snippet)`: nightly `nargo check` output per (package by Nargo version).
- `package_alternatives (package_id, alternative_id)`: symmetric links between entries in the same primitive/use-case space.

**Keyword conventions (no schema change needed, the `package_keywords` table already supports this):**

- `primitive:bls12-381`, `primitive:poseidon`, `primitive:ecdsa`
- `usecase:zkemail`, `usecase:semaphore`, `usecase:verifier-sol`

## What this unlocks

- **Compat badges on every entry:** compiles on latest / old-only / broken
- **"What compiles on Nargo X" filter:** solves the version-pain complaint directly
- **Alternatives on entry pages:** "If you're using X, also consider Y (Z% fewer opcodes)"
- **Automated data via extended scraper:** no manual curation needed

## Migration path

1. **DB migration** (this PR): additive schema deltas.
2. **Extend scraper:** one field to add, mapping `pushed_at` to `last_commit_at`.
3. **New binary `compat-runner`:** nightly cron. For each package: clone, run `nargo check` against target Nargo versions, write results.
4. **Backfill tags:** run `server/scripts/backfill_primitives.sql` to populate `primitive:*` and `usecase:*` keywords.
5. **New API endpoints:** `GET /api/packages/:name/compat`, `GET /api/packages/:name/alternatives`.
6. **Frontend surface updates:** replace the category dropdown with tag search, add compat badges on package cards.
7. **Category removal:** drop `categories` and `package_categories` tables plus the `/api/categories` endpoint once the frontend no longer references them.

Rough scope: **2 to 3 solo weeks** end to end.

## Open questions for reviewers

1. **Does this solve the categorization problem?** Or is there a fourth lens missing?
2. **Which Nargo versions should the compat matrix target?** Suggested defaults: `1.0.0-beta.6` + `latest stable` + `nightly`. Open to others.
3. **Would Noir Labs / Aztec Labs fund this direction?** If yes, I will scope more precisely.

## Prototype status

- [done] GitHub harvest complete, CSVs in `research/harvest/`
- [done] Schema delta migration written, `server/migrations/20260708120000_add_v2_compat_and_alternatives.sql`
- [done] Rust models and query sites carry the new fields, `max_compatible_nargo_version` derived on the fly
- [done] Scraper extension (`pushed_at` populates `last_commit_at`)
- [done] Compat runner MVP (verified 4 of 5 flagship libraries fail on Nargo `1.0.0-beta.11`)
- [done] Primitive / usecase tags backfilled for 101 packages
- [todo] Frontend replaces category dropdown with tag search, renders compat badges
- [todo] Categories dropped from API, DB, and CLI after frontend is switched over

---

_Repo: https://github.com/CECILIA-MULANDI/noir-registry, this document lives at `research/schema_v2.md`._
