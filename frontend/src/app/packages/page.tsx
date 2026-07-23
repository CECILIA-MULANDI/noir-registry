'use client';

import { useState, useEffect, useMemo } from 'react';
import { getPackages, getKeywords } from '../lib/api';
import { Package } from '../lib/types';
import Header from '../components/Header';
import Footer from '../components/Footer';
import PackageCard from '../components/PackageCard';
import PackageSearch from '../components/PackageSearch';
import Link from 'next/link';

type SortOption = 'name' | 'stars' | 'date' | 'updated';

export default function PackagesPage() {
  const [packages, setPackages] = useState<Package[]>([]);
  const [, setKeywords] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [sortBy, setSortBy] = useState<SortOption>('stars');
  const [searchQuery, setSearchQuery] = useState('');
  const [activeKeyword, setActiveKeyword] = useState<string | null>(null);

  useEffect(() => {
    async function fetchAll() {
      try {
        const [pkgData, kwData] = await Promise.all([
          getPackages(),
          getKeywords(),
        ]);
        setPackages(pkgData);
        setKeywords(kwData);
      } catch (error) {
        console.error('Error fetching data:', error);
      } finally {
        setLoading(false);
      }
    }
    fetchAll();
  }, []);

  const filteredAndSortedPackages = useMemo(() => {
    let filtered = packages;

    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (pkg) =>
          pkg.name.toLowerCase().includes(query) ||
          pkg.description?.toLowerCase().includes(query) ||
          pkg.keywords?.some((kw) => kw.toLowerCase().includes(query))
      );
    }

    if (activeKeyword) {
      filtered = filtered.filter((pkg) => pkg.keywords?.includes(activeKeyword));
    }

    return [...filtered].sort((a, b) => {
      switch (sortBy) {
        case 'name':
          return a.name.localeCompare(b.name);
        case 'stars':
          return (b.github_stars || 0) - (a.github_stars || 0);
        case 'date':
          return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
        case 'updated':
          return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime();
        default:
          return 0;
      }
    });
  }, [packages, sortBy, searchQuery, activeKeyword]);

  function handleKeywordClick(kw: string) {
    setActiveKeyword((prev) => (prev === kw ? null : kw));
  }

  function clearFilters() {
    setActiveKeyword(null);
    setSearchQuery('');
  }

  if (loading) {
    return (
      <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-darker)' }}>
        <Header />
        <main className="py-16">
          <div className="max-w-[1200px] mx-auto px-8 text-center">
            <p style={{ color: 'var(--text-secondary)' }}>Loading packages...</p>
          </div>
        </main>
        <Footer />
      </div>
    );
  }

  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-darker)' }}>
      <Header />

      <main className="py-16">
        <div className="max-w-[1200px] mx-auto px-8">
          {/* Back Link */}
          <Link
            href="/"
            className="inline-flex items-center gap-2 text-sm mb-8 no-underline transition-colors hover-text-primary"
            style={{ color: 'var(--text-secondary)' }}
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
            Back to home
          </Link>

          <div>
            {/* Page Header */}
            <div className="mb-6">
              <h1 className="text-4xl font-bold mb-1" style={{ color: 'var(--text-primary)' }}>
                All Packages
              </h1>
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Browse all {packages.length} packages in the Noir registry
              </p>
            </div>

            {/* Search */}
            <div className="mb-4">
              <PackageSearch
                placeholder="Search by name, description, or keyword..."
                onQueryChange={setSearchQuery}
              />
            </div>

            {/* Sort + Active filters row */}
            <div className="flex items-center gap-4 flex-wrap mb-6">
              <span className="text-sm font-medium" style={{ color: 'var(--text-secondary)' }}>
                Sort by:
              </span>
              <div className="flex gap-2 flex-wrap">
                {(['stars', 'name', 'date', 'updated'] as SortOption[]).map((option) => (
                  <button
                    key={option}
                    onClick={() => setSortBy(option)}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${sortBy === option ? 'shadow-md' : 'hover:opacity-80'}`}
                    style={{
                      backgroundColor: sortBy === option ? 'var(--accent-primary)' : 'var(--bg-card)',
                      color: sortBy === option ? 'var(--bg-darker)' : 'var(--text-primary)',
                      border: '1px solid var(--border-color)'
                    }}
                  >
                    {option === 'stars' && 'Most Stars'}
                    {option === 'name' && 'Name'}
                    {option === 'date' && 'Newest'}
                    {option === 'updated' && 'Recently Updated'}
                  </button>
                ))}
              </div>

              {/* Active filter chips */}
              {activeKeyword && (
                <div className="flex items-center gap-2 ml-auto">
                  <span
                    className="px-3 py-1 rounded text-sm font-mono flex items-center gap-1.5"
                    style={{
                      backgroundColor: 'color-mix(in srgb, var(--accent-primary) 15%, transparent)',
                      color: 'var(--accent-primary)',
                      border: '1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)',
                    }}
                  >
                    #{activeKeyword}
                    <button onClick={() => setActiveKeyword(null)} className="hover:opacity-70 leading-none">Clear</button>
                  </span>
                  <button
                    onClick={clearFilters}
                    className="text-xs underline hover:opacity-70"
                    style={{ color: 'var(--text-muted)' }}
                  >
                    Clear all
                  </button>
                </div>
              )}
            </div>

            {/* Results count */}
            {(searchQuery.trim() || activeKeyword) && (
              <p className="text-sm mb-4" style={{ color: 'var(--text-secondary)' }}>
                Found {filteredAndSortedPackages.length}{' '}
                {filteredAndSortedPackages.length === 1 ? 'package' : 'packages'}
                {searchQuery.trim() && ` matching "${searchQuery}"`}
              </p>
            )}

            {/* Package list */}
            {filteredAndSortedPackages.length === 0 ? (
              <div className="text-center py-12">
                <p className="text-lg font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                  No packages found
                </p>
                <p className="text-sm mb-4" style={{ color: 'var(--text-secondary)' }}>
                  Try a different search term or filter
                </p>
                <button
                  onClick={clearFilters}
                  className="text-sm underline"
                  style={{ color: 'var(--accent-primary)' }}
                >
                  Clear filters
                </button>
              </div>
            ) : (
              <div className="space-y-2">
                {filteredAndSortedPackages.map((pkg) => (
                  <PackageCard
                    key={pkg.id}
                    name={pkg.name}
                    version={pkg.latest_version || 'v0.1.0'}
                    keywords={pkg.keywords}
                    onKeywordClick={handleKeywordClick}
                  />
                ))}
              </div>
            )}
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}
