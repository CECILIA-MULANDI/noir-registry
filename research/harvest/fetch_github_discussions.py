import csv
import os
from pathlib import Path

import httpx
from dotenv import load_dotenv

load_dotenv()
TOKEN = os.environ["GITHUB_NOIR_HARVEST_TOKEN"]
OUTPUT_CSV = Path(__file__).parent / "harvest_github_discussions.csv"

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
    "noir-lang/noir-bignum",
    "AztecProtocol/aztec-packages",
    "noir-lang/awesome-noir",
]

QUERY = """
query($owner: String!, $name: String!, $cursor: String) {
  repository(owner: $owner, name: $name) {
    discussions(first: 50, after: $cursor, orderBy: {field: CREATED_AT, direction: DESC}) {
      nodes {
        title
        body
        createdAt
        url
        author { login }
        category { name }
      }
      pageInfo { hasNextPage endCursor }
    }
  }
}
"""


def fetch_all_discussions_for_repo(client: httpx.Client, repo: str) -> list[dict]:
    owner, name = repo.split("/")
    all_discussions = []
    cursor = None
    while True:
        r = client.post(
            "https://api.github.com/graphql",
            json={"query": QUERY, "variables": {"owner": owner, "name": name, "cursor": cursor}},
            headers={"Authorization": f"Bearer {TOKEN}"},
        )
        r.raise_for_status()
        payload = r.json()
        if payload.get("errors"):
            print(f"  ! GraphQL error for {repo}: {payload['errors']}")
            return []
        repo_data = payload["data"]["repository"]
        if repo_data is None:
            print(f"  ! {repo} not found or inaccessible")
            return []
        data = repo_data["discussions"]
        all_discussions.extend(data["nodes"])
        if not data["pageInfo"]["hasNextPage"]:
            break
        cursor = data["pageInfo"]["endCursor"]
    return all_discussions


def find_matches(discussion: dict) -> list[str]:
    haystack = f"{discussion['title']}\n{discussion['body']}".lower()
    return [kw for kw in KEYWORDS if kw.lower() in haystack]


def to_csv_row(d: dict, matches: list[str], repo: str) -> dict:
    author = d["author"]["login"] if d["author"] else "(deleted)"
    category = d["category"]["name"] if d["category"] else "Uncategorized"
    body_snippet = (d["body"] or "").replace("\n", " ").replace("\r", " ").strip()[:300]
    return {
        "source": "github_discussions",
        "repo": repo,
        "category": category,
        "date": d["createdAt"][:10],
        "author": author,
        "title": d["title"],
        "url": d["url"],
        "matched_keywords": "; ".join(matches),
        "body_snippet": body_snippet,
    }


def main() -> None:
    rows = []
    matched_count = 0

    with httpx.Client(timeout=30.0) as client:
        for repo in REPOS:
            print(f"\n=== {repo} ===")
            discussions = fetch_all_discussions_for_repo(client, repo)
            print(f"Fetched {len(discussions)} discussions from {repo}.")

            for d in discussions:
                matches = find_matches(d)
                rows.append(to_csv_row(d, matches, repo))
                if not matches:
                    continue
                matched_count += 1
                author = d["author"]["login"] if d["author"] else "(deleted)"
                category = d["category"]["name"] if d["category"] else "Uncategorized"
                print("-" * 80)
                print(f"[{repo}] [{category}] {d['title']}")
                print(f"  by {author} on {d['createdAt']}")
                print(f"  {d['url']}")
                print(f"  matched: {', '.join(matches)}")

    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)

    print(f"\n{matched_count} of {len(rows)} discussions matched at least one keyword.")
    print(f"CSV written: {OUTPUT_CSV.name} ({len(rows)} rows across {len(REPOS)} repos).")


if __name__ == "__main__":
    main()
