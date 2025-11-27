import { getPackageByName } from '../../lib/api';
import Header from '../../components/Header';
import Footer from '../../components/Footer';
import Link from 'next/link';
import { notFound } from 'next/navigation';

interface PackagePageProps {
  params: Promise<{ name: string }>;
}

export default async function PackagePage({ params }: PackagePageProps) {
  const { name } = await params;
  const pkg = await getPackageByName(name);

  if (!pkg) {
    notFound();
  }

  // Extract GitHub owner and repo from URL
  const githubUrl = pkg.github_repository_url || '';
  const githubMatch = githubUrl.match(/github\.com\/([^\/]+)\/([^\/]+)/);
  const githubOwner = githubMatch ? githubMatch[1] : pkg.owner_github_username || '';
  const githubRepo = githubMatch ? githubMatch[2] : '';

  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
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

          {/* Package Header */}
          <div className="mb-8">
            <div className="flex items-start justify-between mb-4">
              <div className="flex-1">
                <h1 className="text-4xl font-bold mb-2" style={{ color: 'var(--text-primary)' }}>
                  {pkg.name}
                </h1>
                {pkg.description && (
                  <p className="text-lg mb-4" style={{ color: 'var(--text-secondary)' }}>
                    {pkg.description}
                  </p>
                )}
              </div>
              {pkg.owner_avatar_url && (
                <img 
                  src={pkg.owner_avatar_url} 
                  alt={pkg.owner_github_username || 'Owner'}
                  className="w-16 h-16 rounded-full border-2"
                  style={{ borderColor: 'var(--border-color)' }}
                />
              )}
            </div>

            {/* Package Meta Info */}
            <div className="flex flex-wrap gap-6 items-center text-sm" style={{ color: 'var(--text-secondary)' }}>
              {pkg.latest_version && (
                <div className="flex items-center gap-2">
                  <span className="font-semibold">Version:</span>
                  <span className="font-mono px-2 py-1 rounded" style={{ backgroundColor: 'var(--bg-card)', color: 'var(--text-primary)' }}>
                    {pkg.latest_version}
                  </span>
                </div>
              )}
              {pkg.license && (
                <div className="flex items-center gap-2">
                  <span className="font-semibold">License:</span>
                  <span>{pkg.license}</span>
                </div>
              )}
              {pkg.github_stars !== null && pkg.github_stars !== undefined && (
                <div className="flex items-center gap-2">
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                  </svg>
                  <span>{pkg.github_stars.toLocaleString()} stars</span>
                </div>
              )}
              {pkg.total_downloads !== null && pkg.total_downloads !== undefined && (
                <div className="flex items-center gap-2">
                  <span className="font-semibold">Downloads:</span>
                  <span>{pkg.total_downloads.toLocaleString()}</span>
                </div>
              )}
            </div>
          </div>

          {/* Installation Instructions */}
          <div className="mb-8 p-6 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
            <h2 className="text-xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>Installation</h2>
            <div className="bg-black rounded p-4 font-mono text-sm overflow-x-auto">
              <code style={{ color: '#00ff00' }}>
                {`nargo add ${pkg.name}`}
              </code>
            </div>
          </div>

          {/* Links Section */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-8">
            {pkg.github_repository_url && (
              <a
                href={pkg.github_repository_url}
                target="_blank"
                rel="noopener noreferrer"
                className="p-4 rounded-lg no-underline transition-all hover-card flex items-center gap-3"
                style={{ 
                  backgroundColor: 'var(--bg-card)', 
                  border: '1px solid var(--border-color)' 
                }}
              >
                <svg className="w-6 h-6 flex-shrink-0" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                <div>
                  <div className="font-semibold" style={{ color: 'var(--text-primary)' }}>View on GitHub</div>
                  <div className="text-xs" style={{ color: 'var(--text-muted)' }}>
                    {githubOwner && githubRepo ? `${githubOwner}/${githubRepo}` : 'Repository'}
                  </div>
                </div>
              </a>
            )}
            
            {pkg.homepage && (
              <a
                href={pkg.homepage}
                target="_blank"
                rel="noopener noreferrer"
                className="p-4 rounded-lg no-underline transition-all hover-card flex items-center gap-3"
                style={{ 
                  backgroundColor: 'var(--bg-card)', 
                  border: '1px solid var(--border-color)' 
                }}
              >
                <svg className="w-6 h-6 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                </svg>
                <div>
                  <div className="font-semibold" style={{ color: 'var(--text-primary)' }}>Homepage</div>
                  <div className="text-xs truncate" style={{ color: 'var(--text-muted)' }}>
                    {pkg.homepage}
                  </div>
                </div>
              </a>
            )}
          </div>

          {/* Package Info */}
          <div className="p-6 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
            <h2 className="text-xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>Package Information</h2>
            <dl className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {pkg.owner_github_username && (
                <>
                  <dt className="font-semibold" style={{ color: 'var(--text-secondary)' }}>Owner</dt>
                  <dd style={{ color: 'var(--text-primary)' }}>{pkg.owner_github_username}</dd>
                </>
              )}
              {pkg.created_at && (
                <>
                  <dt className="font-semibold" style={{ color: 'var(--text-secondary)' }}>Created</dt>
                  <dd style={{ color: 'var(--text-primary)' }}>
                    {new Date(pkg.created_at).toLocaleDateString('en-US', { 
                      year: 'numeric', 
                      month: 'long', 
                      day: 'numeric' 
                    })}
                  </dd>
                </>
              )}
              {pkg.updated_at && (
                <>
                  <dt className="font-semibold" style={{ color: 'var(--text-secondary)' }}>Last Updated</dt>
                  <dd style={{ color: 'var(--text-primary)' }}>
                    {new Date(pkg.updated_at).toLocaleDateString('en-US', { 
                      year: 'numeric', 
                      month: 'long', 
                      day: 'numeric' 
                    })}
                  </dd>
                </>
              )}
            </dl>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}