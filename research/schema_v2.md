# Noir Registry v2: Schema Design

## Why this exists

In May 2026, Savio told me the categorization in v1 "didn't much make sense." I paused, built more Noir projects to understand the space, then went to look at **how devs actually search for libraries.**

I harvested **1,813 real dev interactions** across:

| Source                                | Rows  |
| ------------------------------------- | ----- |
| `noir-lang/noir` GitHub Discussions   | 131   |
| `noir-lang/noir` issues (most-recent) | 1,000 |
| `AztecProtocol/aztec-packages` issues | 637   |
| `noir-lang/noir-bignum` issues        | 45    |

**444 matched** on library-discovery / primitive / version-pain vocabulary. Raw CSVs live in `research/harvest/`. This document is what I learned and what I want to change.

## Three findings

**1. Devs search by primitive name, not by category.** They don't type "Cryptography"; they type "bls12-381", "elgamal", "poseidon", "ecdsa". A category tree fights this. The v1 categories (Cryptography / Data Structures / Math / Utilities / Zero Knowledge / Circuits / Standards) don't map onto how anyone searches.

**2. Nargo version compat is a chronic, first-class pain, and no registry surfaces it.** Real, recent examples from the harvest:

- [noir-array-helpers v0.30.4 fails to compile due to integer bit width mismatch on latest Noir](https://github.com/noir-lang/noir/issues/12348)
- [Tests fail on latest Nargo nightly release](https://github.com/noir-lang/noir-bignum/issues/262). This is on `noir-bignum` itself. Even the flagship library breaks on nightly.
- [Guidance on Aztec NR compile errors, suspected Noir/Nargo version mismatch](https://github.com/AztecProtocol/aztec-packages/issues/16431). A real dev asking for exactly this.
- [Circuit size blowup from Aztec v3-v4 (root cause potentially in Noir compiler)](https://github.com/noir-lang/noir/issues/12411)

**3. NRG grants are an official demand signal no registry surfaces.** Four rounds, ~$350k allocated. Each round has `[NRG Request]` posts (what the team wants built) + proposal posts (who's building). Nowhere is there a "here's what was asked + here's what got built + here's the current state" view.

## Three lenses, same data

1. **"I need a library for X"**: primitive / use-case tag search
2. **"What did the Noir team ask for + who's building it?"**: NRG round view
3. **"What compiles on my Nargo version?"**: compat matrix / maintenance view

The same underlying entries surface through three different entry points.

## Schema changes (additive, nothing gets deleted)

**Kept as-is:** `packages`, `package_versions`, `package_keywords`, `categories`, `package_categories`, `users`.

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
- `nrg:round-1`, `nrg:round-4`

**Categories:** demoted from primary organizing axis. Frontend surfaces tags first; the existing 7 categories become secondary metadata visible on entry pages. **No data loss, no rows deleted.**

## What this unlocks

- **Compat badges on every entry:** compiles on latest / old-only / broken
- **"What compiles on Nargo X" filter:** solves the version-pain complaint directly
- **NRG round view:** "Here's what NRG#4 asked for, here's the 12 proposals, here's who shipped, here's current maintenance state"
- **Alternatives on entry pages:** "If you're using X, also consider Y (Z% fewer opcodes)"
- **Automated data via extended scraper:** no manual curation needed

## Migration path

1. **DB migration** (this PR): additive schema deltas, zero downtime, zero data loss.
2. **Extend scraper:** one field to add, mapping `pushed_at` to `last_commit_at`.
3. **New binary `compat-runner`:** nightly cron. For each package: clone, run `nargo check` against target Nargo versions, write results.
4. **New API endpoints:** `GET /api/packages/:name/compat`, `GET /api/packages/:name/alternatives`, `GET /api/nrg/:round`.
5. **NRG backfill script:** read `research/harvest/harvest_github_discussions.csv`, tag NRG#1-#4 entries.
6. **Frontend surface updates:** demote category dropdown, elevate tag search, add compat badges, add NRG-round view.

Rough scope: **2 to 3 solo weeks** for schema + backfill + compat runner MVP + surface changes.

## Open questions for @Savio-Sou

1. **Does this solve the categorization problem you flagged in May?** Or is there a fourth lens I'm missing?
2. **Which Nargo versions should the compat matrix target?** Suggested defaults: `1.0.0-beta.6` + `latest stable` + `nightly`. Open to others.
3. **Is NRG#5 planned?** I'd love to align the schema before it launches so we can surface the round in real time.
4. **Would you (or Noir Labs / Aztec Labs) fund this direction?** If yes, I'll scope more precisely.

## Prototype status

- [done] Discord / GitHub harvest complete, CSVs in `research/harvest/`
- [done] Schema delta migration written, `server/migrations/20260708120000_add_v2_compat_and_alternatives.sql`
- [wip] Compat runner MVP
- [todo] Scraper extension (fetch `pushed_at`)
- [todo] NRG round backfill from harvest data
- [todo] API + frontend surface changes

---

_Repo: https://github.com/CECILIA-MULANDI/noir-registry, this document lives at `research/schema_v2.md`._
