'use client';

import { useState, useEffect, useMemo } from 'react';
import { getPackages, getKeywords, getCategories } from '../lib/api';
import { Category, Package } from '../lib/types';
import Header from '../components/Header';
import Footer from '../components/Footer';
import PackageCard from '../components/PackageCard';
import Link from 'next/link';

type SortOption = 'name' | 'stars' | 'date' | 'updated';

export default function PackagesPage() {
  const [packages, setPackages] = useState<Package[]>([]);
  const [keywords, setKeywords] = useState<string[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState(true);
  const [sortBy, setSortBy] = useState<SortOption>('stars');
  const [searchQuery, setSearchQuery] = useState('');
  const [activeKeyword, setActiveKeyword] = useState<string | null>(null);
  const [activeCategory, setActiveCategory] = useState<string | null>(null);

  useEffect(() => {
    async function fetchAll() {
      try {
        const [pkgData, kwData, catData] = await Promise.all([
          getPackages(),
          getKeywords(),
          getCategories(),
        ]);
        setPackages(pkgData);
        setKeywords(kwData);
        setCategories(catData);
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

    if (activeCategory) {
      // Category filter is done server-side via API; this is a client-side fallback
      // (works when packages already loaded include category info)
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
  }, [packages, sortBy, searchQuery, activeKeyword, activeCategory]);

  function handleKeywordClick(kw: string) {
    setActiveKeyword((prev) => (prev === kw ? null : kw));
    setActiveCategory(null);
  }

  function handleCategoryClick(slug: string) {
    setActiveCategory((prev) => (prev === slug ? null : slug));
    setActiveKeyword(null);
  }

  function clearFilters() {
    setActiveKeyword(null);
    setActiveCategory(null);
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

          <div className="flex gap-8">
            {/* Sidebar */}
            <aside className="w-56 flex-shrink-0">
              {/* Categories */}
              {categories.length > 0 && (
                <div className="mb-6">
                  <h3 className="text-xs font-semibold uppercase tracking-wider mb-3"
                    style={{ color: 'var(--text-muted)' }}>
                    Categories
                  </h3>
                  <ul className="space-y-1">
                    {categories.map((cat) => (
                      <li key={cat.slug}>
                        <button
                          onClick={() => handleCategoryClick(cat.slug)}
                          className="w-full text-left px-3 py-1.5 rounded text-sm transition-all"
                          style={{
                            backgroundColor: activeCategory === cat.slug
                              ? 'color-mix(in srgb, var(--accent-primary) 15%, transparent)'
                              : 'transparent',
                            color: activeCategory === cat.slug
                              ? 'var(--accent-primary)'
                              : 'var(--text-secondary)',
                            fontWeight: activeCategory === cat.slug ? 600 : 400,
                          }}
                        >
                          {cat.name}
                        </button>
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {/* Keywords */}
              {keywords.length > 0 && (
                <div>
                  <h3 className="text-xs font-semibold uppercase tracking-wider mb-3"
                    style={{ color: 'var(--text-muted)' }}>
                    Keywords
                  </h3>
                  <div className="flex flex-wrap gap-1.5">
                    {keywords.map((kw) => (
                      <button
                        key={kw}
                        onClick={() => handleKeywordClick(kw)}
                        className="px-2 py-0.5 rounded text-xs font-mono transition-opacity hover:opacity-70"
                        style={{
                          backgroundColor: activeKeyword === kw
                            ? 'var(--accent-primary)'
                            : 'color-mix(in srgb, var(--accent-primary) 15%, transparent)',
                          color: activeKeyword === kw
                            ? 'var(--bg-darker)'
                            : 'var(--accent-primary)',
                          border: '1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)',
                        }}
                      >
                        {kw}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </aside>

            {/* Main content */}
            <div className="flex-1 min-w-0">
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
              <div className="relative mb-4">
                <input
                  type="text"
                  placeholder="Search by name, description, or keyword..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full px-5 py-3 pr-12 text-base rounded-lg outline-none transition-all focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20"
                  style={{
                    backgroundColor: 'var(--bg-card)',
                    border: '1px solid var(--border-color)',
                    color: 'var(--text-primary)'
                  }}
                />
                <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none"
                  style={{ color: 'var(--text-muted)' }}>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                  </svg>
                </div>
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
                      {option === 'stars' && '⭐ Most Stars'}
                      {option === 'name' && '🔤 Name'}
                      {option === 'date' && '📅 Newest'}
                      {option === 'updated' && '🔄 Recently Updated'}
                    </button>
                  ))}
                </div>

                {/* Active filter chips */}
                {(activeKeyword || activeCategory) && (
                  <div className="flex items-center gap-2 ml-auto">
                    {activeKeyword && (
                      <span
                        className="px-3 py-1 rounded text-sm font-mono flex items-center gap-1.5"
                        style={{
                          backgroundColor: 'color-mix(in srgb, var(--accent-primary) 15%, transparent)',
                          color: 'var(--accent-primary)',
                          border: '1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)',
                        }}
                      >
                        #{activeKeyword}
                        <button onClick={() => setActiveKeyword(null)} className="hover:opacity-70 leading-none">✕</button>
                      </span>
                    )}
                    {activeCategory && (
                      <span
                        className="px-3 py-1 rounded text-sm flex items-center gap-1.5"
                        style={{
                          backgroundColor: 'color-mix(in srgb, var(--accent-primary) 15%, transparent)',
                          color: 'var(--accent-primary)',
                          border: '1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)',
                        }}
                      >
                        {categories.find((c) => c.slug === activeCategory)?.name ?? activeCategory}
                        <button onClick={() => setActiveCategory(null)} className="hover:opacity-70 leading-none">✕</button>
                      </span>
                    )}
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
              {(searchQuery.trim() || activeKeyword || activeCategory) && (
                <p className="text-sm mb-4" style={{ color: 'var(--text-secondary)' }}>
                  Found {filteredAndSortedPackages.length}{' '}
                  {filteredAndSortedPackages.length === 1 ? 'package' : 'packages'}
                  {searchQuery.trim() && ` matching "${searchQuery}"`}
                </p>
              )}

              {/* Package list */}
              {filteredAndSortedPackages.length === 0 ? (
                <div className="text-center py-12">
                  <div className="text-6xl mb-4">📦</div>
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
        </div>
      </main>

      <Footer />
    </div>
  );
}
