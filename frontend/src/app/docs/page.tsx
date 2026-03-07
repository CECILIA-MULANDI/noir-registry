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
                  nargo add package-name
                </code>
              </div>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                <strong>Note:</strong> After installing <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>nargo-add</code>, you can use <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>nargo add</code> directly. The tool also works with <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>nargo-add package-name</code> if you prefer.
              </p>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                Replace <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>package-name</code> with the actual package name from the registry.
              </p>
            </div>
          </section>

          {/* Removing Packages */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Removing Packages
            </h2>
            <div className="space-y-4">
              <p style={{ color: 'var(--text-secondary)' }}>
                To remove a package from your Noir project, use:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  nargo remove package-name
                </code>
              </div>
              <p style={{ color: 'var(--text-secondary)' }}>
                You can also remove multiple packages at once:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  nargo remove package-one package-two
                </code>
              </div>
              <p style={{ color: 'var(--text-secondary)' }}>
                To also delete the cached source files from <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>~/nargo</code>, use the <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>--clean</code> flag:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  nargo remove package-name --clean
                </code>
              </div>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                <strong>Note:</strong> Without <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>--clean</code>, only the dependency entry in{' '}
                <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>Nargo.toml</code>{' '}
                is removed. Cached source files are left in place for other projects that may use them.
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

          {/* Publishing a Package */}
          <section className="mb-12">
            <h2 className="text-2xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
              Publishing a Package
            </h2>
            <div className="space-y-4">
              <p style={{ color: 'var(--text-secondary)' }}>
                To publish a package, you need an API key tied to your GitHub account. The registry verifies that you own the GitHub repository before publishing.
              </p>

              <h3 className="text-lg font-semibold mt-6" style={{ color: 'var(--text-primary)' }}>Step 1 — Get an API key</h3>
              <p style={{ color: 'var(--text-secondary)' }}>
                Authenticate with your GitHub account to receive an API key:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  {`curl -X POST https://noir-registry-production-229a.up.railway.app/api/auth/github \\
  -H "Content-Type: application/json" \\
  -d '{"github_token": "your_github_personal_access_token"}'`}
                </code>
              </div>
              <p className="text-sm" style={{ color: 'var(--text-muted)' }}>
                The response will include your <code className="px-1 py-0.5 rounded font-mono text-xs" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>api_key</code>. Keep it safe.
              </p>

              <h3 className="text-lg font-semibold mt-6" style={{ color: 'var(--text-primary)' }}>Step 2 — Publish your package</h3>
              <p style={{ color: 'var(--text-secondary)' }}>
                Send a POST request with your package details:
              </p>
              <div className="bg-black rounded-lg p-4 font-mono text-sm overflow-x-auto">
                <code style={{ color: '#00ff00' }}>
                  {`curl -X POST https://noir-registry-production-229a.up.railway.app/api/packages/publish \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "name": "my-package",
    "description": "A useful Noir library",
    "github_repository_url": "https://github.com/your-username/my-package",
    "version": "0.1.0",
    "license": "MIT",
    "keywords": ["crypto", "hashing"],
    "category": "cryptography"
  }'`}
                </code>
              </div>

              <div className="p-4 rounded-lg mt-4" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
                <h4 className="font-semibold mb-3" style={{ color: 'var(--text-primary)' }}>Fields</h4>
                <dl className="space-y-2 text-sm">
                  {[
                    ['name', 'required', 'Alphanumeric, hyphens/underscores, max 50 chars'],
                    ['github_repository_url', 'required', 'Must be a repo you own on GitHub'],
                    ['description', 'optional', 'Short description of the package'],
                    ['version', 'optional', 'Semver string e.g. 0.1.0'],
                    ['license', 'optional', 'e.g. MIT, Apache-2.0'],
                    ['keywords', 'optional', 'Array of strings for discoverability'],
                    ['category', 'optional', 'One of: cryptography, data-structures, math, utilities, zero-knowledge, circuits, standards'],
                  ].map(([field, req, desc]) => (
                    <div key={field} className="flex gap-3">
                      <code className="px-1 py-0.5 rounded font-mono text-xs flex-shrink-0" style={{ backgroundColor: 'var(--bg-darker)', color: 'var(--text-primary)' }}>{field}</code>
                      <span className="text-xs px-1 rounded flex-shrink-0" style={{ color: req === 'required' ? 'var(--accent-primary)' : 'var(--text-muted)', border: '1px solid currentColor' }}>{req}</span>
                      <span style={{ color: 'var(--text-secondary)' }}>{desc}</span>
                    </div>
                  ))}
                </dl>
              </div>
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
                    curl https://noir-registry-production-229a.up.railway.app/api/packages
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
                    curl https://noir-registry-production-229a.up.railway.app/api/packages/package-name
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
                    curl https://noir-registry-production-229a.up.railway.app/api/search?q=cryptography
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