import csv
from collections import Counter
from pathlib import Path

HERE = Path(__file__).parent

INTENT_PHRASES = {
    "is there a", "looking for", "library for", "lib for", "package for",
    "anyone built", "anyone have", "any noir lib", "existing implementation",
    "alternative to", "how do i verify",
}

VERSION_PHRASES = {
    "doesn't compile", "fails to compile", "outdated", "deprecated",
    "0.17", "breaking change", "port to noir", "upgrade nargo",
}


def load(name: str) -> list[dict]:
    with (HERE / name).open() as f:
        return list(csv.DictReader(f))


def kws(row: dict) -> set[str]:
    raw = row.get("matched_keywords", "")
    return {kw for kw in raw.split("; ") if kw}


def main() -> None:
    discussions = load("harvest_github_discussions.csv")
    issues = load("harvest_github_issues.csv")
    for r in discussions:
        r["source_type"] = "discussion"
    for r in issues:
        r["source_type"] = "issue"

    all_rows = discussions + issues
    matched = [r for r in all_rows if r.get("matched_keywords")]

    print(f"TOTAL ROWS: {len(all_rows)}   MATCHED: {len(matched)}\n")

    # keyword frequency across all matches
    kw_counter: Counter[str] = Counter()
    for r in matched:
        for kw in kws(r):
            kw_counter[kw] += 1
    print("=== TOP 30 KEYWORDS (all matches) ===")
    for kw, n in kw_counter.most_common(30):
        print(f"  {n:4d}  {kw}")

    # repo distribution
    repo_counter = Counter(r["repo"] for r in matched)
    print("\n=== MATCHES BY REPO ===")
    for repo, n in repo_counter.most_common():
        print(f"  {n:4d}  {repo}")

    # intent rows — where someone is actively searching for a library
    intent_rows = [r for r in matched if kws(r) & INTENT_PHRASES]
    intent_rows.sort(key=lambda r: len(kws(r)), reverse=True)
    print(f"\n=== INTENT-BEARING ROWS: {len(intent_rows)} ===")
    for r in intent_rows[:35]:
        title = r.get("title", "")
        ctx = r.get("category") or r.get("state") or ""
        intent_hits = sorted(kws(r) & INTENT_PHRASES)
        body = (r.get("body_snippet") or "")[:220]
        print(f"\n[{r['source_type']}][{r['repo']}][{ctx}] {title}")
        print(f"  intent: {intent_hits}")
        print(f"  all matches: {r.get('matched_keywords', '')}")
        print(f"  {r.get('url', '')}")
        if body:
            print(f"  body: {body}")

    # keyword frequency INSIDE intent rows only — this is what people actually search for
    kw_intent: Counter[str] = Counter()
    for r in intent_rows:
        for kw in kws(r) - INTENT_PHRASES:
            kw_intent[kw] += 1
    print("\n=== TOP KEYWORDS AMONG INTENT-BEARING ROWS (primitives + use-cases people ask for) ===")
    for kw, n in kw_intent.most_common(25):
        print(f"  {n:4d}  {kw}")

    # version pain
    version_rows = [r for r in matched if kws(r) & VERSION_PHRASES]
    version_rows.sort(key=lambda r: r.get("date", ""), reverse=True)
    print(f"\n=== VERSION-PAIN ROWS: {len(version_rows)} (top 15 by date) ===")
    for r in version_rows[:15]:
        title = r.get("title", "")
        vhits = sorted(kws(r) & VERSION_PHRASES)
        print(f"\n[{r['source_type']}][{r['repo']}] {title}")
        print(f"  version-pain hits: {vhits}")
        print(f"  {r.get('url', '')}")


if __name__ == "__main__":
    main()
