"use client";

import { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import { getPackages } from "../lib/api";
import type { Package } from "../lib/types";

interface PackageSearchProps {
  placeholder?: string;
  variant?: "hero" | "compact";
  onQueryChange?: (query: string) => void;
  autoFocusShortcut?: boolean;
}

const MAX_SUGGESTIONS = 8;

// Lower score wins. Name matches rank above keyword matches, which rank above description matches.
function scorePackage(pkg: Package, q: string): number | null {
  const name = pkg.name.toLowerCase();
  if (name.startsWith(q)) return 0;
  if (name.includes(q)) return 1;
  const kws = (pkg.keywords ?? []).map((k) => k.toLowerCase());
  if (kws.some((k) => k.includes(q))) return 2;
  const desc = (pkg.description ?? "").toLowerCase();
  if (desc.includes(q)) return 3;
  return null;
}

export default function PackageSearch({
  placeholder = "Search packages by name, primitive, or use case",
  variant = "compact",
  onQueryChange,
  autoFocusShortcut = false,
}: PackageSearchProps) {
  const [query, setQuery] = useState("");
  const [packages, setPackages] = useState<Package[]>([]);
  const [showDropdown, setShowDropdown] = useState(false);
  const [highlightedIndex, setHighlightedIndex] = useState(-1);
  const containerRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  useEffect(() => {
    getPackages().then(setPackages).catch(() => {});
  }, []);

  useEffect(() => {
    if (!autoFocusShortcut) return;
    const handleKeyPress = (e: KeyboardEvent) => {
      if (
        (e.key === "s" || e.key === "/") &&
        document.activeElement?.tagName !== "INPUT"
      ) {
        e.preventDefault();
        inputRef.current?.focus();
      }
    };
    window.addEventListener("keydown", handleKeyPress);
    return () => window.removeEventListener("keydown", handleKeyPress);
  }, [autoFocusShortcut]);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setShowDropdown(false);
      }
    };
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  }, []);

  const matches: Package[] = (() => {
    const q = query.trim().toLowerCase();
    if (q.length === 0) return [];
    const scored = packages
      .map((pkg) => ({ pkg, score: scorePackage(pkg, q) }))
      .filter((e): e is { pkg: Package; score: number } => e.score !== null);
    scored.sort((a, b) => a.score - b.score);
    return scored.slice(0, MAX_SUGGESTIONS).map((e) => e.pkg);
  })();

  function updateQuery(v: string) {
    setQuery(v);
    setShowDropdown(true);
    setHighlightedIndex(-1);
    onQueryChange?.(v);
  }

  function goToPackage(name: string) {
    router.push(`/packages/${name}`);
    setShowDropdown(false);
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (!showDropdown || matches.length === 0) return;
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setHighlightedIndex((i) => Math.min(i + 1, matches.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setHighlightedIndex((i) => Math.max(i - 1, 0));
    } else if (e.key === "Enter") {
      e.preventDefault();
      const pick =
        highlightedIndex >= 0 ? matches[highlightedIndex] : matches[0];
      if (pick) goToPackage(pick.name);
    } else if (e.key === "Escape") {
      setShowDropdown(false);
    }
  }

  const inputClass =
    variant === "hero"
      ? "w-full px-5 py-4 pr-12 text-base rounded-full outline-none transition-all focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20"
      : "w-full px-5 py-3 pr-12 text-base rounded-lg outline-none transition-all focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20";

  return (
    <div className="relative w-full" ref={containerRef}>
      <input
        ref={inputRef}
        type="text"
        placeholder={placeholder}
        value={query}
        onChange={(e) => updateQuery(e.target.value)}
        onFocus={() => query.trim() && setShowDropdown(true)}
        onKeyDown={handleKeyDown}
        className={inputClass}
        style={{
          backgroundColor: "var(--bg-card)",
          border: "1px solid var(--border-color)",
          color: "var(--text-primary)",
        }}
      />
      <div
        className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none"
        style={{ color: "var(--text-muted)" }}
      >
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
          />
        </svg>
      </div>

      {showDropdown && matches.length > 0 && (
        <div
          className="absolute top-full left-0 right-0 mt-2 rounded-lg overflow-hidden z-50 max-h-96 overflow-y-auto shadow-lg"
          style={{
            backgroundColor: "var(--bg-card)",
            border: "1px solid var(--border-color)",
          }}
        >
          {matches.map((pkg, idx) => (
            <button
              key={pkg.id}
              type="button"
              onClick={() => goToPackage(pkg.name)}
              onMouseEnter={() => setHighlightedIndex(idx)}
              className="w-full px-4 py-3 text-left block transition-colors"
              style={{
                borderBottom:
                  idx === matches.length - 1
                    ? "none"
                    : "1px solid var(--border-color)",
                color: "var(--text-primary)",
                backgroundColor:
                  idx === highlightedIndex
                    ? "color-mix(in srgb, var(--accent-primary) 10%, transparent)"
                    : "transparent",
              }}
            >
              <div className="font-semibold text-sm">{pkg.name}</div>
              {pkg.description && (
                <div
                  className="text-xs mt-1 line-clamp-1"
                  style={{ color: "var(--text-muted)" }}
                >
                  {pkg.description}
                </div>
              )}
              {pkg.keywords && pkg.keywords.length > 0 && (
                <div className="flex flex-wrap gap-1 mt-2">
                  {pkg.keywords.slice(0, 4).map((k) => (
                    <span
                      key={k}
                      className="px-2 py-0.5 rounded text-xs font-mono"
                      style={{
                        backgroundColor:
                          "color-mix(in srgb, var(--accent-primary) 15%, transparent)",
                        color: "var(--accent-primary)",
                        border:
                          "1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)",
                      }}
                    >
                      {k}
                    </span>
                  ))}
                </div>
              )}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
