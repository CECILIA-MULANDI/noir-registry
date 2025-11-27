import Link from 'next/link';
import Header from '../../components/Header';
import Footer from '../../components/Footer';

export default function NotFound() {
  return (
    <div className="min-h-screen" style={{ backgroundColor: 'var(--bg-dark)' }}>
      <Header />
      
      <main className="py-16">
        <div className="max-w-[1200px] mx-auto px-8 text-center">
          <div className="text-6xl mb-4">📦</div>
          <h1 className="text-4xl font-bold mb-4" style={{ color: 'var(--text-primary)' }}>
            Package Not Found
          </h1>
          <p className="text-lg mb-8" style={{ color: 'var(--text-secondary)' }}>
            The package you're looking for doesn't exist or has been removed.
          </p>
          <Link 
            href="/"
            className="inline-block px-6 py-3 rounded-lg font-semibold no-underline transition-all hover-bg-accent"
            style={{ backgroundColor: 'var(--accent-primary)', color: 'var(--bg-darker)' }}
          >
            Back to Home
          </Link>
        </div>
      </main>

      <Footer />
    </div>
  );
}