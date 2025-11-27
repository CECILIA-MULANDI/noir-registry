import Link from 'next/link';

export default function Header() {
  return (
    <header className="sticky top-0 z-50 backdrop-blur-sm" style={{ backgroundColor: 'var(--bg-darker)', borderBottom: '1px solid var(--border-color)' }}>
      <div className="max-w-[1200px] mx-auto px-8 py-5">
        <div className="flex justify-between items-center">
          <Link href="/" className="flex items-center gap-3 text-xl font-bold no-underline hover:opacity-80 transition-opacity" style={{ color: 'var(--text-primary)' }}>
            <div className="w-8 h-8 border rounded flex items-center justify-center" style={{ backgroundColor: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
              <span className="text-white font-bold text-lg">N</span>
            </div>
            <span>noir Registry</span>
          </Link>
          <nav className="flex gap-10 items-center">
            <Link href="/packages" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
              Browse All Packages
            </Link>
            <Link href="/docs" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
              Documentation
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
}