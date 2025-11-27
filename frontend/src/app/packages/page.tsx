'use client';

import { useState, useEffect, useMemo } from 'react';
import { getPackages } from '../lib/api';
import { Package } from '../lib/types';
import Header from '../components/Header';
import Footer from '../components/Footer';
import PackageCard from '../components/PackageCard';
import Link from 'next/link';

type SortOption = 'name' | 'stars' | 'date' | 'updated';

export default function PackagesPage() {
  const [packages, setPackages] = useState<Package[]>([]);
  const [loading, setLoading] = useState(true);
  const [sortBy, setSortBy] = useState<SortOption>('stars');
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    async function fetchPackages() {
      try {
        const data = await getPackages();
        setPackages(data);
      } catch (error) {
        console.error('Error fetching packages:', error);
      } finally {
        setLoading(false);
      }
    }
    fetchPackages();
  }, []);

  // Filter and sort packages
  const filteredAndSortedPackages = useMemo(() => {
    let filtered = packages;

    // Apply search filter
    if (searchQuery.trim()) {
      const query = searchQuery.toLowerCase();
      filtered = packages.filter(
        (pkg) =>
          pkg.name.toLowerCase().includes(query) ||
          pkg.description?.toLowerCase().includes(query)
      );
    }

    // Apply sorting
    const sorted = [...filtered].sort((a, b) => {
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

    return sorted;
  }, [packages, sortBy, searchQuery]);

  if (loading) {
    return (
      <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
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
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
      <Header />

      <main className="py-16">
        <div className="max-w-[1200px] mx-auto px-8">
          {/* Page Header */}
          <div className="mb-8">
            <h1 className="text-4xl font-bold mb-2" style={{ color: 'var(--text-primary)' }}>
              All Packages
            </h1>
            <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
              Browse all {packages.length} packages in the Noir registry
            </p>
          </div>

          {/* Search and Sort Controls */}
          <div className="mb-8 space-y-4">
            {/* Search Bar */}
            <div className="relative">
              <input
                type="text"
                placeholder="Search packages by name or description..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full px-5 py-3 pr-12 text-base rounded-lg outline-none transition-all focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20"
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

            {/* Sort Controls */}
            <div className="flex items-center gap-4 flex-wrap">
              <span className="text-sm font-medium" style={{ color: 'var(--text-secondary)' }}>
                Sort by:
              </span>
              <div className="flex gap-2 flex-wrap">
                {(['stars', 'name', 'date', 'updated'] as SortOption[]).map((option) => (
                  <button
                    key={option}
                    onClick={() => setSortBy(option)}
                    className={`px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                      sortBy === option
                        ? 'shadow-md'
                        : 'hover:opacity-80'
                    }`}
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
            </div>

            {/* Results Count */}
            {searchQuery.trim() && (
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Found {filteredAndSortedPackages.length} {filteredAndSortedPackages.length === 1 ? 'package' : 'packages'}
                {searchQuery.trim() && ` matching "${searchQuery}"`}
              </p>
            )}
          </div>

          {/* Packages List */}
          {filteredAndSortedPackages.length === 0 ? (
            <div className="text-center py-12">
              <div className="text-6xl mb-4">📦</div>
              <p className="text-lg font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                No packages found
              </p>
              <p className="text-sm mb-4" style={{ color: 'var(--text-secondary)' }}>
                {searchQuery.trim()
                  ? 'Try a different search term'
                  : 'No packages available'}
              </p>
              {searchQuery.trim() && (
                <button
                  onClick={() => setSearchQuery('')}
                  className="text-sm underline"
                  style={{ color: 'var(--accent-primary)' }}
                >
                  Clear search
                </button>
              )}
            </div>
          ) : (
            <div className="space-y-2">
              {filteredAndSortedPackages.map((pkg) => (
                <PackageCard
                  key={pkg.id}
                  id={pkg.id}
                  name={pkg.name}
                  version={pkg.latest_version || 'v0.1.0'}
                />
              ))}
            </div>
          )}
        </div>
      </main>

      <Footer />
    </div>
  );
}