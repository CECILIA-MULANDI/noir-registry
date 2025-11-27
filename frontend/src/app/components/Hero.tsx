'use client';

import { useState, useEffect, useRef } from 'react';
import Link from 'next/link';

export default function Hero() {
  const [searchQuery, setSearchQuery] = useState('');
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Keyboard shortcut: 'S' or '/' to focus search
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if ((e.key === 's' || e.key === '/') && document.activeElement?.tagName !== 'INPUT') {
        e.preventDefault();
        searchInputRef.current?.focus();
      }
    };

    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, []);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchQuery.trim()) {
      window.location.href = `/search?q=${encodeURIComponent(searchQuery.trim())}`;
    }
  };

  return (
    <section className="py-20" style={{ backgroundColor: 'var(--bg-darker)', borderBottom: '1px solid var(--border-color)' }}>
      <div className="max-w-[1200px] mx-auto px-8">
        <h1 className="text-5xl md:text-6xl font-bold mb-12 text-center leading-tight" style={{ color: 'var(--text-primary)' }}>
          The Noir community's package registry
        </h1>

        {/* Search Bar and Action Buttons */}
        <div className="max-w-[800px] mx-auto mb-12">
          <form onSubmit={handleSearch} className="flex flex-col sm:flex-row gap-4 items-center">
            <div className="flex-1 w-full relative">
              <input
                ref={searchInputRef}
                type="text"
                placeholder="Type 'S' or '/' to search"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full px-5 py-4 pr-12 text-base rounded-lg outline-none transition-all focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20"
                style={{ 
                  backgroundColor: 'var(--bg-card)', 
                  border: '1px solid var(--border-color)',
                  color: 'var(--text-primary)'
                }}
              />
              <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none" style={{ color: 'var(--text-muted)' }}>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </div>
            </div>
            <div className="flex gap-3">
              <Link href="https://noir-lang.org/docs/getting_started/installation" target="_blank" rel="noopener noreferrer" className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent" style={{ backgroundColor: 'var(--accent-primary)', color: 'var(--bg-darker)' }}>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
                </svg>
                <span>Install Nargo</span>
              </Link>
              <Link href="https://noir-lang.org/docs/getting_started" target="_blank" rel="noopener noreferrer" className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent" style={{ backgroundColor: 'var(--accent-primary)', color: 'var(--bg-darker)' }}>
                <span>Getting Started</span>
              </Link>
            </div>
          </form>
        </div>
      </div>
    </section>
  );
}