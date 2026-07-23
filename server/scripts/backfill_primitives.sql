-- Backfill primitive:* and usecase:* keywords for the 39 packages scraped
-- from awesome-noir as of July 2026. Addresses Finding 1 from research/schema_v2.md:
-- devs search by primitive name, not by v1 category. Tags are added alongside
-- existing category rows; nothing is deleted.
--
-- Idempotent. Safe to re-run.
-- Run manually:
--   psql "$DATABASE_URL_CLEAN" -f server/scripts/backfill_primitives.sql

BEGIN;

-- Hashes
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:poseidon', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'Poseidon' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:keccak256', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'Keccak256' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:mimc', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'MiMC' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:sha256', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'SHA256' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:sha512', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'SHA512' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ripemd160', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'RIPEMD160' ON CONFLICT DO NOTHING;

-- Signatures
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ecdsa', 'primitive:secp256k1', 'usecase:signature']::TEXT[]) AS kw
WHERE p.name = 'ECDSA' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:rsa', 'usecase:signature']::TEXT[]) AS kw
WHERE p.name = 'RSA' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:eddsa', 'usecase:signature']::TEXT[]) AS kw
WHERE p.name = 'EdDSA' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:schnorr', 'usecase:signature']::TEXT[]) AS kw
WHERE p.name = 'Schnorr' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:eip-712', 'usecase:signature', 'usecase:ethereum']::TEXT[]) AS kw
WHERE p.name = 'EIP-712' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ecrecover', 'primitive:secp256k1', 'usecase:ethereum']::TEXT[]) AS kw
WHERE p.name = 'ECrecover' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:qes', 'usecase:signature', 'usecase:zk']::TEXT[]) AS kw
WHERE p.name = 'zkQES' ON CONFLICT DO NOTHING;

-- Elliptic curves and key exchange
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:bls12-381', 'primitive:bn254', 'primitive:elliptic-curve']::TEXT[]) AS kw
WHERE p.name = 'BigCurve' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ecdh', 'primitive:secp256k1', 'usecase:key-exchange']::TEXT[]) AS kw
WHERE p.name = 'ECDH' ON CONFLICT DO NOTHING;

-- Big numbers and numeric types
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:bignum', 'primitive:u256', 'primitive:field-arithmetic']::TEXT[]) AS kw
WHERE p.name = 'BigNum' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:floating-point', 'primitive:ieee754']::TEXT[]) AS kw
WHERE p.name = 'IEEE754 Floating-point' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:fixed-point', 'primitive:wad', 'usecase:defi']::TEXT[]) AS kw
WHERE p.name = 'wad.nr Fixed-point' ON CONFLICT DO NOTHING;

-- Merkle trees and inclusion / storage proofs
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:merkle-tree', 'usecase:proof-of-inclusion']::TEXT[]) AS kw
WHERE p.name = 'ZK-Kit: Merkle Trees' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:merkle-tree', 'usecase:tooling']::TEXT[]) AS kw
WHERE p.name = 'Merkle Tree Generator' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:mpt', 'usecase:ethereum', 'usecase:storage-proof']::TEXT[]) AS kw
WHERE p.name = 'Ethereum MPT Proof' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:ethereum', 'usecase:storage-proof']::TEXT[]) AS kw
WHERE p.name = 'Ethereum Storage Proof' ON CONFLICT DO NOTHING;

-- Encoding and parsing
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:base64', 'usecase:encoding']::TEXT[]) AS kw
WHERE p.name = 'Base64' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:json', 'usecase:parsing']::TEXT[]) AS kw
WHERE p.name = 'JSON parser' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:xpath', 'usecase:parsing']::TEXT[]) AS kw
WHERE p.name = 'XPath 2.0 Functions' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:regex', 'usecase:zkemail']::TEXT[]) AS kw
WHERE p.name = 'zkRegEx' ON CONFLICT DO NOTHING;

-- Data structures and algorithms
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:string', 'usecase:search']::TEXT[]) AS kw
WHERE p.name = 'String Search' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:sort', 'usecase:algorithm']::TEXT[]) AS kw
WHERE p.name = 'Sort' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:sparse-array', 'usecase:data-structure']::TEXT[]) AS kw
WHERE p.name = 'Sparse Array' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:date', 'usecase:datetime']::TEXT[]) AS kw
WHERE p.name = 'Noir Dates' ON CONFLICT DO NOTHING;

-- Advanced crypto (FHE and MPC)
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:rlwe', 'usecase:fhe']::TEXT[]) AS kw
WHERE p.name = 'RLWE Gadgets' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:multiparty-computation', 'usecase:proving']::TEXT[]) AS kw
WHERE p.name = 'coSNARKs' ON CONFLICT DO NOTHING;

-- Utility and aggregate libraries
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:utility']::TEXT[]) AS kw
WHERE p.name = 'nodash' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:zk-primitives', 'primitive:merkle-tree']::TEXT[]) AS kw
WHERE p.name = 'ZK Kit Noir' ON CONFLICT DO NOTHING;

-- Applications built on Noir (not libraries themselves)
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:identity']::TEXT[]) AS kw
WHERE p.name = 'Self' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:social']::TEXT[]) AS kw
WHERE p.name = 'anon.world' ON CONFLICT DO NOTHING;

-- Tooling
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:tree-sitter']::TEXT[]) AS kw
WHERE p.name = 'tree_sitter_noir' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:debugger']::TEXT[]) AS kw
WHERE p.name = 'CodeTracer' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:profiler']::TEXT[]) AS kw
WHERE p.name = 'Noir + Barretenberg Profiler' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:poseidon2', 'primitive:poseidon', 'usecase:tooling']::TEXT[]) AS kw
WHERE p.name = 'Poseidon2 in TypeScript' ON CONFLICT DO NOTHING;

-- Extended backfill: covers the remaining packages in the DB beyond the
-- initial 39 scraper set. Skipped where the package name is opaque
-- (Apertrue, hunter, lampe, Mezcal, my-real-package, noirupi, op_rand,
-- SKProof, Terry Escape, ZK-Flexor, zk-mutant).

-- Additional hashes and HMACs
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:hmac', 'primitive:hash']::TEXT[]) AS kw
WHERE p.name = 'Noir HMAC' ON CONFLICT DO NOTHING;

-- Authentication and JWT
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:jwt', 'usecase:jwt', 'usecase:authentication']::TEXT[]) AS kw
WHERE p.name = 'JWT' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:webauthn', 'usecase:authentication']::TEXT[]) AS kw
WHERE p.name = 'WebAuthn/Passkeys' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:authentication']::TEXT[]) AS kw
WHERE p.name = 'zkLogin' ON CONFLICT DO NOTHING;

-- Additional pairings and curves
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:bls12-381', 'primitive:pairing', 'primitive:elliptic-curve']::TEXT[]) AS kw
WHERE p.name = 'Pairing over BLS12-381' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:bn254', 'primitive:elliptic-curve']::TEXT[]) AS kw
WHERE p.name = 'Hydra for BN254' ON CONFLICT DO NOTHING;

-- Symmetric encryption
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:aes', 'usecase:encryption']::TEXT[]) AS kw
WHERE p.name = 'AES' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:chacha20', 'usecase:encryption']::TEXT[]) AS kw
WHERE p.name = 'ChaCha20 Implementation' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:elgamal', 'usecase:encryption']::TEXT[]) AS kw
WHERE p.name = 'ElGamal Encryption' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ecies', 'usecase:encryption']::TEXT[]) AS kw
WHERE p.name = 'ECIES' ON CONFLICT DO NOTHING;

-- Numeric types and math libraries
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:complex-number']::TEXT[]) AS kw
WHERE p.name = 'Complex Numbers' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:fraction', 'primitive:rational']::TEXT[]) AS kw
WHERE p.name = 'Fraction' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:fixed-point']::TEXT[]) AS kw
WHERE p.name = 'Fixed Point Library' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:fixed-point']::TEXT[]) AS kw
WHERE p.name = 'Fixed Point Library for scale 2^-16' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:ieee754', 'primitive:floating-point']::TEXT[]) AS kw
WHERE p.name = 'IEEE754' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:floating-point']::TEXT[]) AS kw
WHERE p.name = 'ZKFloat' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:matrix', 'usecase:linear-algebra']::TEXT[]) AS kw
WHERE p.name = 'Matrix Operations' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:quantized', 'usecase:ml']::TEXT[]) AS kw
WHERE p.name = 'Quantized arithmetic' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:statistics']::TEXT[]) AS kw
WHERE p.name = 'Statistical Library' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:convolution', 'usecase:signal-processing']::TEXT[]) AS kw
WHERE p.name = 'Convolution' ON CONFLICT DO NOTHING;

-- Additional merkle trees and nullifiers
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:merkle-tree']::TEXT[]) AS kw
WHERE p.name = 'Merkle Tree' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:indexed-merkle-tree', 'primitive:merkle-tree']::TEXT[]) AS kw
WHERE p.name = 'Indexed Merkle Tree' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:nullifier', 'primitive:rln']::TEXT[]) AS kw
WHERE p.name = 'Rate Limiting Nullifiers' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:plume', 'primitive:nullifier']::TEXT[]) AS kw
WHERE p.name = 'PLUME' ON CONFLICT DO NOTHING;

-- Additional encoding and byte manipulation
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:base64', 'usecase:encoding']::TEXT[]) AS kw
WHERE p.name = 'Noir Base64 Library' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:bytes', 'usecase:conversion']::TEXT[]) AS kw
WHERE p.name = 'U(int)2B(ytes)' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:string', 'usecase:utility']::TEXT[]) AS kw
WHERE p.name = 'String Utils' ON CONFLICT DO NOTHING;

-- Semaphore and anonymity
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:semaphore', 'usecase:anonymity']::TEXT[]) AS kw
WHERE p.name = 'Noir Semaphore' ON CONFLICT DO NOTHING;

-- Additional data structures
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:linked-list', 'usecase:data-structure']::TEXT[]) AS kw
WHERE p.name = 'Lib_LinkList' ON CONFLICT DO NOTHING;

-- Additional date/time
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:datetime', 'usecase:datetime']::TEXT[]) AS kw
WHERE p.name = 'DateTimeNr' ON CONFLICT DO NOTHING;

-- Randomness
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:csprng', 'primitive:random']::TEXT[]) AS kw
WHERE p.name = 'Cryptographically Secure Pseudo-Random Number Generator' ON CONFLICT DO NOTHING;

-- ML and zkML
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:ml', 'usecase:zkml']::TEXT[]) AS kw
WHERE p.name = 'ML' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:zkml', 'usecase:ml']::TEXT[]) AS kw
WHERE p.name = 'zkML-Noir' ON CONFLICT DO NOTHING;

-- Chain-specific proofs
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:aztec', 'usecase:storage-proof']::TEXT[]) AS kw
WHERE p.name = 'Aztec Storage proofs' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:bitcoin', 'usecase:proving']::TEXT[]) AS kw
WHERE p.name = 'bitcoin-prover' ON CONFLICT DO NOTHING;

-- Identity / KYC
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:aadhaar', 'usecase:identity', 'usecase:kyc']::TEXT[]) AS kw
WHERE p.name = 'Anon-Aadhaar' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:identity', 'usecase:kyc']::TEXT[]) AS kw
WHERE p.name = 'Rarimo' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:identity', 'usecase:social']::TEXT[]) AS kw
WHERE p.name = 'Noir Social Verify' ON CONFLICT DO NOTHING;

-- Governance / voting
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:voting', 'usecase:governance']::TEXT[]) AS kw
WHERE p.name = 'Nouns Anonymous Voting' ON CONFLICT DO NOTHING;

-- DeFi
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:defi', 'usecase:bonds']::TEXT[]) AS kw
WHERE p.name = 'Private Tokenised Bonds' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:defi', 'usecase:reimbursement', 'usecase:zkemail']::TEXT[]) AS kw
WHERE p.name = 'z-imburse' ON CONFLICT DO NOTHING;

-- Messaging / private communication
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:messaging', 'usecase:private']::TEXT[]) AS kw
WHERE p.name = 'StealthNote' ON CONFLICT DO NOTHING;

-- Games
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:gaming']::TEXT[]) AS kw
WHERE p.name = 'ZK Blackjack' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:gaming', 'usecase:anticheat']::TEXT[]) AS kw
WHERE p.name = 'ZK-AntiCheat' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:gaming']::TEXT[]) AS kw
WHERE p.name = 'zk-hangman-noir' ON CONFLICT DO NOTHING;

-- Recursion / proving
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:recursion', 'usecase:proving']::TEXT[]) AS kw
WHERE p.name = 'noir-recursive' ON CONFLICT DO NOTHING;

-- Standard library
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['primitive:stdlib']::TEXT[]) AS kw
WHERE p.name = 'Standard Library' ON CONFLICT DO NOTHING;

-- Tooling (starters, editors, benchmarks, examples, formal verification)
INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:examples']::TEXT[]) AS kw
WHERE p.name = 'Circuit Examples' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:fuzzer']::TEXT[]) AS kw
WHERE p.name = 'Circuzz fuzzer' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:foundry']::TEXT[]) AS kw
WHERE p.name = 'foundry-noir-helper' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:hardhat']::TEXT[]) AS kw
WHERE p.name = 'hardhat-noir-starter' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:editor']::TEXT[]) AS kw
WHERE p.name = 'Neovim Plugin' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:benchmarking']::TEXT[]) AS kw
WHERE p.name = 'Noir Benchmark CLI' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:examples']::TEXT[]) AS kw
WHERE p.name = 'Noir Examples' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:starter']::TEXT[]) AS kw
WHERE p.name = 'noir-library-starter' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:metrics']::TEXT[]) AS kw
WHERE p.name = 'noir-metrics' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:react-native']::TEXT[]) AS kw
WHERE p.name = 'noir-react-native-starter' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:learning']::TEXT[]) AS kw
WHERE p.name = 'Noirlings' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:formal-verification']::TEXT[]) AS kw
WHERE p.name = 'rocq-of-noir' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:benchmarking', 'primitive:rsa']::TEXT[]) AS kw
WHERE p.name = 'RSA Benchmarks' ON CONFLICT DO NOTHING;

INSERT INTO package_keywords (package_id, keyword)
SELECT p.id, kw FROM packages p, unnest(ARRAY['usecase:tooling', 'usecase:editor']::TEXT[]) AS kw
WHERE p.name = 'Zed Plugin' ON CONFLICT DO NOTHING;

COMMIT;
