import csv
import os
from pathlib import Path

import httpx
from dotenv import load_dotenv

load_dotenv()
TOKEN = os.environ["GITHUB_NOIR_HARVEST_TOKEN"]
OUTPUT_CSV = Path(__file__).parent / "harvest_github_issues.csv"

KEYWORDS = [
    "is there a", "anyone built", "anyone have", "looking for",
    "library for", "lib for", "package for", "any noir lib",
    "how do i verify", "existing implementation",

    "ecdsa", "schnorr", "rsa", "bls", "eddsa", "ed25519", "secp256k1",
    "poseidon", "keccak", "sha256", "blake", "pedersen", "mimc",
    "elgamal", "aes",
    "bn254", "bls12-381", "baby jubjub", "grumpkin",
    "bignum", "u256", "u128 overflow",
    "kzg", "plonk", "groth16", "halo2", "ultrahonk",

    "jwt", "oauth", "merkle proof", "range proof", "set membership",
    "nullifier", "commitment", "recursive proof", "proof aggregation",
    "verifier.sol", "on-chain verification", "private voting",
    "zk identity", "kyc", "zkml",

    "doesn't compile", "fails to compile", "outdated", "deprecated",
    "0.17", "breaking change", "port to noir", "upgrade nargo",

    "instead of", "alternative to", "difference between", "which is better",
]

REPOS = [
    "noir-lang/noir",
    "AztecProtocol/aztec-packages",
    "noir-lang/noir-bignum",
]

MAX_ISSUES_PER_REPO = 1000


def fetch_issues_for_repo(client: httpx.Client, repo: str) -> list[dict]:
    all_issues = []
    page = 1
    while len(all_issues) < MAX_ISSUES_PER_REPO:
        r = client.get(
            f"https://api.github.com/repos/{repo}/issues",
            params={
                "state": "all",
                "sort": "created",
                "direction": "desc",
                "per_page": 100,
                "page": page,
            },
            headers={
                "Authorization": f"Bearer {TOKEN}",
                "Accept": "application/vnd.github+json",
            },
        )
        if r.status_code != 200:
            print(f"  ! {repo} returned {r.status_code}: {r.text[:200]}")
            return all_issues
        batch = r.json()
        if not batch:
            break
        issues = [i for i in batch if "pull_request" not in i]
        all_issues.extend(issues)
        if len(batch) < 100:
            break
        page += 1
    return all_issues[:MAX_ISSUES_PER_REPO]


def find_matches(issue: dict) -> list[str]:
    haystack = f"{issue['title']}\n{issue.get('body') or ''}".lower()
    return [kw for kw in KEYWORDS if kw.lower() in haystack]


def to_csv_row(issue: dict, matches: list[str], repo: str) -> dict:
    author = issue["user"]["login"] if issue.get("user") else "(deleted)"
    labels = "; ".join(label["name"] for label in issue.get("labels", []))
    body_snippet = (issue.get("body") or "").replace("\n", " ").replace("\r", " ").strip()[:300]
    return {
        "source": "github_issues",
        "repo": repo,
        "state": issue["state"],
        "labels": labels,
        "date": issue["created_at"][:10],
        "author": author,
        "title": issue["title"],
        "url": issue["html_url"],
        "matched_keywords": "; ".join(matches),
        "body_snippet": body_snippet,
    }


def main() -> None:
    rows = []
    matched_count = 0
    with httpx.Client(timeout=30.0) as client:
        for repo in REPOS:
            print(f"\n=== {repo} ===")
            issues = fetch_issues_for_repo(client, repo)
            print(f"Fetched {len(issues)} issues from {repo}.")
            for issue in issues:
                matches = find_matches(issue)
                rows.append(to_csv_row(issue, matches, repo))
                if matches:
                    matched_count += 1

    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

    print(f"\n{matched_count} of {len(rows)} issues matched at least one keyword.")
    print(f"CSV written: {OUTPUT_CSV.name} ({len(rows)} rows across {len(REPOS)} repos).")


if __name__ == "__main__":
    main()
