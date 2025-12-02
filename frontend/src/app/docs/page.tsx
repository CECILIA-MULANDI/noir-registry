import Header from '../components/Header';
import Footer from '../components/Footer';
import Link from 'next/link';

export default function DocsPage() {
  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-darker)' }}>
      <Header />

      <main className="py-16">
        <div className="max-w-[1000px] mx-auto px-8">
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

          <h1 className="text-4xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
            Documentation
          </h1>
          <p className="text-lg mb-12" style={{ color: 'var(--text-secondary)' }}>
            Learn how to use the Noir Package Registry
          </p>

          {/* Getting Started */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Getting Started
            </h2>
            <div className="space-y-4" style={{ color: 'var(--text-secondary)' }}>
              <p>
                The Noir Package Registry is a centralized repository for Noir packages, similar to npm for JavaScript or crates.io for Rust.
              </p>
              <p>
                Browse packages, search for functionality, and install them in your Noir projects.
              </p>
            </div>
          </section>

          {/* Installing Packages */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Installing Packages
            </h2>
            <div className="space-y-4">
              <p style={{ color: 'var(--text-secondary)' }}>
                First, install the <code className="px-2 py-1 rounded font-mono text-sm" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>nargo-add</code> CLI tool:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  cargo install nargo-add
                </code>
              </div>
              <p style={{ color: 'var(--text-secondary)' }}>
                Then, to install a package in your Noir project, use:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  nargo-add package-name
                </code>
              </div>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                Replace <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>package-name</code> with the actual package name from the registry.
              </p>
            </div>
          </section>

          {/* Searching Packages */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Searching Packages
            </h2>
            <div className="space-y-4" style={{ color: 'var(--text-secondary)' }}>
              <p>
                You can search for packages in several ways:
              </p>
              <ul className="list-disc list-inside space-y-2 ml-4">
                <li>Use the search bar on the homepage</li>
                <li>Press <kbd className="px-2 py-1 rounded text-xs font-mono" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>S</kbd> or <kbd className="px-2 py-1 rounded text-xs font-mono" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>/</kbd> to focus the search</li>
                <li>Browse all packages from the navigation menu</li>
                <li>View featured packages on the homepage</li>
              </ul>
            </div>
          </section>

          {/* API Documentation */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              API Documentation
            </h2>
            <div className="space-y-6">
              <div className="p-4 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
                <h3 className="font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                  GET /api/packages
                </h3>
                <p className="text-sm mb-2" style={{ color: 'var(--text-secondary)' }}>
                  Get all packages
                </p>
                <div className="bg-black rounded p-2 font-mono text-xs overflow-x-auto">
                  <code style={{ color: '#00ff00' }}>
                    curl http://109.205.177.65/api/packages
                  </code>
                </div>
              </div>

              <div className="p-4 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
                <h3 className="font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                  GET /api/packages/:name
                </h3>
                <p className="text-sm mb-2" style={{ color: 'var(--text-secondary)' }}>
                  Get a specific package by name
                </p>
                <div className="bg-black rounded p-2 font-mono text-xs overflow-x-auto">
                  <code style={{ color: '#00ff00' }}>
                    curl http://109.205.177.65/api/packages/package-name
                  </code>
                </div>
              </div>

              <div className="p-4 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
                <h3 className="font-semibold mb-2" style={{ color: 'var(--text-primary)' }}>
                  GET /api/search?q=query
                </h3>
                <p className="text-sm mb-2" style={{ color: 'var(--text-secondary)' }}>
                  Search packages by name or description
                </p>
                <div className="bg-black rounded p-2 font-mono text-xs overflow-x-auto">
                  <code style={{ color: '#00ff00' }}>
                    curl http://109.205.177.65/api/search?q=cryptography
                  </code>
                </div>
              </div>
            </div>
          </section>

          {/* External Resources */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Learn More
            </h2>
            <div className="space-y-3">
              <a
                href="https://noir-lang.org/docs"
                target="_blank"
                rel="noopener noreferrer"
                className="block p-4 rounded-lg no-underline transition-all hover-card"
                style={{
                  backgroundColor: 'var(--bg-card)',
                  border: '1px solid var(--border-color)'
                }}
              >
                <div className="font-semibold mb-1" style={{ color: 'var(--text-primary)' }}>
                  Noir Language Documentation
                </div>
                <div className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                  Official documentation for the Noir programming language
                </div>
              </a>

              {/* <a
                href="https://noir-lang.org/docs/getting_started"
                target="_blank"
                rel="noopener noreferrer"
                className="block p-4 rounded-lg no-underline transition-all hover-card"
                style={{
                  backgroundColor: 'var(--bg-card)',
                  border: '1px solid var(--border-color)'
                }}
              >
                <div className="font-semibold mb-1" style={{ color: 'var(--text-primary)' }}>
                  Getting Started with Noir
                </div>
                <div className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                  Learn the basics of Noir development
                </div>
              </a> */}

              <a
                href="https://github.com/noir-lang/awesome-noir"
                target="_blank"
                rel="noopener noreferrer"
                className="block p-4 rounded-lg no-underline transition-all hover-card"
                style={{
                  backgroundColor: 'var(--bg-card)',
                  border: '1px solid var(--border-color)'
                }}
              >
                <div className="font-semibold mb-1" style={{ color: 'var(--text-primary)' }}>
                  Awesome Noir
                </div>
                <div className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                  Curated list of Noir resources and packages
                </div>
              </a>
            </div>
          </section>
        </div>
      </main>

      <Footer />
    </div>
  );
}