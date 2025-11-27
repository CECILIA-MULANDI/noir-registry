export default function Footer() {
    return (
      <footer className="border-t py-10 mt-16 text-center text-sm" style={{ borderColor: 'var(--border-color)', backgroundColor: 'var(--bg-dark)', color: 'var(--text-muted)' }}>
        <div className="max-w-[1200px] mx-auto px-8 flex items-center justify-center gap-3">
          <div className="text-4xl font-bold opacity-40" style={{ color: 'var(--text-primary)' }}>N</div>
          <p>Noir Registry - The package registry for the Noir programming language</p>
        </div>
      </footer>
    );
  }