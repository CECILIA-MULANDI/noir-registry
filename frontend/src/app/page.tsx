import { getPackages } from './lib/api';
import { Package } from './lib/types';
import Header from './components/Header';
import Hero from './components/Hero';
import StatsSection from './components/StatsSection';
import PackageCard from './components/PackageCard';
import Footer from './components/Footer';

export default async function HomePage() {
  const packages = await getPackages();

  // Sort packages according to different criteria
  const newPackages = [...packages]
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    .slice(0, 4);
  
  const mostDownloaded = [...packages]
    .sort((a, b) => (b.github_stars || 0) - (a.github_stars || 0))
    .slice(0, 4);

  const justUpdated = [...packages]
    .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
    .slice(0, 4);

  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
      <Header />
      <Hero />
      <StatsSection packageCount={packages.length} />
      
      {/* Package Lists Section */}
      <section className="py-16" style={{ backgroundColor: 'var(--bg-dark)' }}>
        <div className="max-w-[1200px] mx-auto px-8">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10">
            {/* New Packages */}
            <div>
              <h3 className="text-lg font-bold mb-6 uppercase tracking-wider" style={{ color: 'var(--text-primary)' }}>NEW PACKAGES</h3>
              <div className="space-y-2">
                {newPackages.length === 0 ? (
                  <p className="text-sm" style={{ color: 'var(--text-muted)' }}>No packages yet</p>
                ) : (
                  newPackages.map((pkg) => (
                    <PackageCard
                      key={pkg.id}
                      id={pkg.id}
                      name={pkg.name}
                      version={pkg.latest_version || 'v0.1.0'}
                    />
                  ))
                )}
              </div>
            </div>

            {/* Most Downloaded */}
            <div>
              <h3 className="text-lg font-bold mb-6 uppercase tracking-wider" style={{ color: 'var(--text-primary)' }}>MOST DOWNLOADED</h3>
              <div className="space-y-2">
                {mostDownloaded.length === 0 ? (
                  <p className="text-sm" style={{ color: 'var(--text-muted)' }}>No packages yet</p>
                ) : (
                  mostDownloaded.map((pkg) => (
                    <PackageCard
                      key={pkg.id}
                      id={pkg.id}
                      name={pkg.name}
                      version={pkg.latest_version || 'v0.1.0'}
                    />
                  ))
                )}
              </div>
            </div>

            {/* Just Updated */}
            <div>
              <h3 className="text-lg font-bold mb-6 uppercase tracking-wider" style={{ color: 'var(--text-primary)' }}>JUST UPDATED</h3>
              <div className="space-y-2">
                {justUpdated.length === 0 ? (
                  <p className="text-sm" style={{ color: 'var(--text-muted)' }}>No packages yet</p>
                ) : (
                  justUpdated.map((pkg) => (
                    <PackageCard
                      key={pkg.id}
                      id={pkg.id}
                      name={pkg.name}
                      version={pkg.latest_version || 'v0.1.0'}
                    />
                  ))
                )}
              </div>
            </div>
          </div>
        </div>
      </section>

      <Footer />
    </div>
  );
}