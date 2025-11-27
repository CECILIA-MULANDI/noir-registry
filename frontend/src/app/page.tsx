
interface Package {
  id: number;
  name: string;
  description: string | null;
  latest_version: string | null;
  github_stars: number | null;
  created_at: string;
  updated_at: string;
}
async function getPackages(): Promise<Package[]> {
  try {
    const res = await fetch('http://localhost:8080/api/packages', {
      // We can always fetch fresh data
      cache: 'no-store'
    });
    if (!res.ok) {
      // Try to get error message from response (could be JSON or text)
      let errorText = res.statusText;
      try {
        const contentType = res.headers.get('content-type');
        if (contentType?.includes('application/json')) {
          const errorJson = await res.json();
          errorText = errorJson.error || JSON.stringify(errorJson);
        } else {
          errorText = await res.text() || res.statusText;
        }
      } catch {
        // If parsing fails, use status text
        errorText = res.statusText;
      }
      console.error(`Failed to fetch packages: ${res.status} ${res.statusText}`);
      console.error('Error details:', errorText);
      return [];
    }
    return res.json();

  } catch (error) {
    // Backend might not be running - return empty array gracefully
    console.warn('Backend not available or error fetching packages:', error);
    return [];
  }
}

export default async function HomePage() {

  const packages = await getPackages();
  // We can now sort the packages according to different criteria
  const newPackages = [...packages].sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()).slice(0, 4);
  const mostDownloaded = [...packages]
    .sort((a, b) => (b.github_stars || 0) - (a.github_stars || 0))
    .slice(0, 4);

  const justUpdated = [...packages]
    .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
    .slice(0, 4);
  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
      {/* Header */}
      <header className="sticky top-0 z-50 backdrop-blur-sm" style={{ backgroundColor: 'var(--bg-darker)', borderBottom: '1px solid var(--border-color)' }}>
        <div className="max-w-[1200px] mx-auto px-8 py-5">
          <div className="flex justify-between items-center">
            <a href="/" className="flex items-center gap-3 text-xl font-bold no-underline hover:opacity-80 transition-opacity" style={{ color: 'var(--text-primary)' }}>
              <div className="w-8 h-8 border rounded flex items-center justify-center" style={{ backgroundColor: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
                <span className="text-white font-bold text-lg">N</span>
              </div>
              <span>noir Registry</span>
            </a>
            <nav className="flex gap-10 items-center">
              <a href="#" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
                Browse All Packages
              </a>
              <a href="#" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
                Documentation
              </a>
            </nav>
          </div>
        </div>
      </header>
      {/* Hero Section */}
      <section className="py-20" style={{ backgroundColor: 'var(--bg-darker)', borderBottom: '1px solid var(--border-color)' }}>
        <div className="max-w-[1200px] mx-auto px-8">
          <h1 className="text-5xl md:text-6xl font-bold mb-12 text-center leading-tight" style={{ color: 'var(--text-primary)' }}>
            The Noir community's package registry
          </h1>

          {/* Search Bar and Action Buttons */}
          <div className="max-w-[800px] mx-auto mb-12">
            <div className="flex flex-col sm:flex-row gap-4 items-center">
              <div className="flex-1 w-full relative">
                <input
                  type="text"
                  placeholder="Type 'S' or '/' to search"
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
                <a href="#" className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent" style={{ backgroundColor: 'var(--accent-primary)', color: 'var(--bg-darker)' }}>
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
                  </svg>
                  <span>Install Nargo</span>
                </a>
                <a href="#" className="px-6 py-4 rounded-lg font-semibold no-underline transition-all inline-flex items-center gap-2 shadow-md hover:shadow-lg transform hover:-translate-y-0.5 hover-bg-accent" style={{ backgroundColor: 'var(--accent-primary)', color: 'var(--bg-darker)' }}>
                  <span>Getting Started</span>
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>
      {/* Stats Section */}
      <section className="py-16" style={{ borderBottom: '1px solid var(--border-color)', backgroundColor: 'var(--bg-dark)' }}>
        <div className="max-w-[1200px] mx-auto px-8">
          <p className="max-w-[700px] mx-auto text-center text-base leading-relaxed mb-12" style={{ color: 'var(--text-secondary)' }}>
            Instantly publish your packages and install them. Use the API to interact
            and find out more information about available packages. Become a
            contributor and enhance the site with your work.
          </p>

          <div className="flex flex-col sm:flex-row justify-center gap-12 sm:gap-20 items-center">
            <div className="flex items-center gap-5">
              <div className="text-5xl">📦</div>
              <div>
                <div className="text-4xl font-bold" style={{ color: 'var(--text-primary)' }}>1,234,567</div>
                <div className="text-sm mt-1.5 font-medium" style={{ color: 'var(--text-secondary)' }}>Downloads</div>
              </div>
            </div>
            <div className="h-16 w-px hidden sm:block" style={{ backgroundColor: 'var(--border-color)' }}></div>
            <div className="flex items-center gap-5">
              <div className="text-5xl">📚</div>
              <div>
                <div className="text-4xl font-bold" style={{ color: 'var(--text-primary)' }}>{packages.length}</div>
                <div className="text-sm mt-1.5 font-medium" style={{ color: 'var(--text-secondary)' }}>Packages in stock</div>
              </div>
            </div>
          </div>
        </div>
      </section>

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
                    <a
                      key={pkg.id}
                      href={`/packages/${pkg.name}`}
                      className="px-5 py-4 rounded-lg flex justify-between items-center cursor-pointer transition-all no-underline block group shadow-sm hover:shadow-md hover-card"
                      style={{ 
                        backgroundColor: 'var(--bg-card)', 
                        border: '1px solid var(--border-color)' 
                      }}
                    >
                      <span className="font-semibold text-sm transition-colors group-hover-text-accent" style={{ color: 'var(--text-primary)' }}>{pkg.name}</span>
                      <div className="flex items-center gap-3">
                        <span className="text-xs font-mono" style={{ color: 'var(--text-muted)' }}>{pkg.latest_version || 'v0.1.0'}</span>
                        <span className="text-lg font-bold transition-colors hover-arrow" style={{ color: 'var(--text-muted)' }}>›</span>
                      </div>
                    </a>
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
                    <a
                      key={pkg.id}
                      href={`/packages/${pkg.name}`}
                      className="px-5 py-4 rounded-lg flex justify-between items-center cursor-pointer transition-all no-underline block group shadow-sm hover:shadow-md hover-card"
                      style={{ 
                        backgroundColor: 'var(--bg-card)', 
                        border: '1px solid var(--border-color)' 
                      }}
                    >
                      <span className="font-semibold text-sm transition-colors group-hover-text-accent" style={{ color: 'var(--text-primary)' }}>{pkg.name}</span>
                      <div className="flex items-center gap-3">
                        <span className="text-xs font-mono" style={{ color: 'var(--text-muted)' }}>{pkg.latest_version || 'v0.1.0'}</span>
                        <span className="text-lg font-bold transition-colors hover-arrow" style={{ color: 'var(--text-muted)' }}>›</span>
                      </div>
                    </a>
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
                    <a
                      key={pkg.id}
                      href={`/packages/${pkg.name}`}
                      className="px-5 py-4 rounded-lg flex justify-between items-center cursor-pointer transition-all no-underline block group shadow-sm hover:shadow-md hover-card"
                      style={{ 
                        backgroundColor: 'var(--bg-card)', 
                        border: '1px solid var(--border-color)' 
                      }}
                    >
                      <span className="font-semibold text-sm transition-colors group-hover-text-accent" style={{ color: 'var(--text-primary)' }}>{pkg.name}</span>
                      <div className="flex items-center gap-3">
                        <span className="text-xs font-mono" style={{ color: 'var(--text-muted)' }}>{pkg.latest_version || 'v0.1.0'}</span>
                        <span className="text-lg font-bold transition-colors hover-arrow" style={{ color: 'var(--text-muted)' }}>›</span>
                      </div>
                    </a>
                  ))
                )}
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="border-t py-10 mt-16 text-center text-sm" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-dark)', color: 'var(--text-muted)' }}>
        <div className="max-w-[1200px] mx-auto px-8 flex items-center justify-center gap-3">
          <div className="text-4xl font-bold opacity-40" style={{ color: 'var(--text-primary)' }}>N</div>
          <p>Noir Registry - The package registry for the Noir programming language</p>
        </div>
      </footer>
    </div>
  );
}

