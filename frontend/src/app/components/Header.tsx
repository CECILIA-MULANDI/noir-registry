import Link from 'next/link';

export default function Header() {
  return (
    <header className="sticky top-0 z-50 backdrop-blur-sm" style={{ backgroundColor: 'var(--bg-darker)', borderBottom: '1px solid var(--border-color)' }}>
      <div className="max-w-[1200px] mx-auto px-4 sm:px-6 md:px-8 py-4 sm:py-5">
        <div className="flex justify-between items-center gap-4">
          <Link href="/" className="flex items-center gap-2 sm:gap-3 text-lg sm:text-xl font-bold no-underline hover:opacity-80 transition-opacity" style={{ color: 'var(--text-primary)' }}>
            <div className="w-7 h-7 sm:w-8 sm:h-8 border rounded flex items-center justify-center" style={{ backgroundColor: 'var(--bg-card)', borderColor: 'var(--border-color)' }}>
              <span className="text-white font-bold text-base sm:text-lg">N</span>
            </div>
            <span>Noir Registry</span>
          </Link>
          <nav className="flex gap-4 sm:gap-6 md:gap-10 items-center">
            <Link href="/packages" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
              <span className="hidden md:inline">Browse All Packages</span>
              <span className="md:hidden">Browse</span>
            </Link>
            <Link href="/docs" className="text-sm font-medium no-underline transition-colors hover-text-primary" style={{ color: 'var(--text-secondary)' }}>
              <span className="hidden md:inline">Documentation</span>
              <span className="md:hidden">Docs</span>
            </Link>
          </nav>
        </div>
      </div>
    </header>
  );
}