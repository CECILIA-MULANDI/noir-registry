"use client";

import Link from "next/link";
import PackageSearch from "./PackageSearch";

export default function Hero() {
  return (
    <section
      className="py-20"
      style={{
        backgroundColor: "var(--bg-darker)",
        borderBottom: "1px solid var(--border-color)",
      }}
    >
      <div className="max-w-[1200px] mx-auto px-8">
        <h1
          className="text-5xl md:text-6xl font-bold mb-12 text-center leading-tight"
          style={{ color: "var(--text-primary)" }}
        >
          The Noir community's package registry
        </h1>

        <div className="max-w-[800px] mx-auto mb-12">
          <div className="flex flex-col gap-4 items-center">
            <div className="w-full">
              <PackageSearch
                variant="hero"
                placeholder="Type 'S' or '/' to search"
                autoFocusShortcut
              />
            </div>
            <div className="flex sm:flex-row gap-3 justify-center w-full">
              <Link
                href="https://noir-lang.org/docs/"
                target="_blank"
                rel="noopener noreferrer"
                className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent"
                style={{
                  backgroundColor: "var(--accent-primary)",
                  color: "var(--bg-darker)",
                }}
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
                  />
                </svg>
                <span>Install Nargo</span>
              </Link>
              <Link
                href="/docs"
                className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent"
                style={{
                  backgroundColor: "var(--accent-primary)",
                  color: "var(--bg-darker)",
                }}
              >
                <span>Getting Started</span>
              </Link>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
