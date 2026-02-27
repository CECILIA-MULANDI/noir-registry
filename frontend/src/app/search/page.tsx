import { searchPackages } from '../lib/api';
import { Package } from '../lib/types';
import Header from '../components/Header';
import Footer from '../components/Footer';
import PackageCard from '../components/PackageCard';

interface SearchPageProps {
  searchParams: Promise<{ q?: string }>;
}

export default async function SearchPage({ searchParams }: SearchPageProps) {
  const params = await searchParams;
  const query = params.q || '';
  const packages: Package[] = query.trim() ? await searchPackages(query) : [];

  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
      <Header />
      
      <main className="py-16">
        <div className="max-w-[1200px] mx-auto px-8">
          {/* Search Header */}
          <div className="mb-8">
            <h1 className="text-3xl font-bold mb-2" style={{ color: 'var(--text-primary)' }}>
              {query.trim() ? `Search results for "${query}"` : 'Search Packages'}
            </h1>
            {query.trim() && (
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Found {packages.length} {packages.length === 1 ? 'package' : 'packages'}
              </p>
            )}
          </div>

          {/* Results */}
          {!query.trim() ? (
            <div className="text-center py-12">
              <p className="text-lg mb-4" style={{ color: 'var(--text-secondary)' }}>
                Enter a search query to find packages
              </p>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                Try searching by package name or description
              </p>
            </div>
          ) : packages.length === 0 ? (
            <div className="text-center py-12">
              <div className="text-6xl mb-4">🔍</div>
              <p className="text-lg font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                No packages found
              </p>
              <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                Try a different search term or browse all packages
              </p>
            </div>
          ) : (
            <div className="space-y-2">
              {packages.map((pkg) => (
                <PackageCard
                  key={pkg.id}
                  name={pkg.name}
                  version={pkg.latest_version || 'v0.1.0'}
                  keywords={pkg.keywords}
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