interface StatsSectionProps {
    packageCount: number;
  }
  
  export default function StatsSection({ packageCount }: StatsSectionProps) {
    return (
      <section className="py-16" style={{ borderBottom: '1px solid var(--border-color)', backgroundColor: 'var(--bg-darker)' }}>
        <div className="max-w-[1200px] mx-auto px-8">
          <p className="max-w-[700px] mx-auto text-center text-base leading-relaxed mb-8" style={{ color: 'var(--text-secondary)' }}>
            Instantly publish your packages and install them. Use the API to interact
            and find out more information about available packages. Become a
            contributor and enhance the site with your work.
          </p>

          {/* nargo-add Installation Instructions */}
          <div className="max-w-[600px] mx-auto mb-12 p-6 rounded-lg" style={{ backgroundColor: 'var(--bg-card)', border: '1px solid var(--border-color)' }}>
            <p className="text-sm font-semibold mb-3 text-center" style={{ color: 'var(--text-primary)' }}>
              Install packages with nargo-add:
            </p>
            <div className="flex flex-col sm:flex-row gap-3 items-center justify-center">
              <code className="px-4 py-2 rounded text-sm font-mono" style={{ backgroundColor: 'var(--bg-darker)', color: 'var(--text-primary)', border: '1px solid var(--border-color)' }}>
                cargo install nargo-add
              </code>
              <span className="text-xs" style={{ color: 'var(--text-muted)' }}>Then use: <code className="px-2 py-1 rounded" style={{ backgroundColor: 'var(--bg-darker)', color: 'var(--text-primary)' }}>nargo-add package-name</code></span>
            </div>
          </div>
  
          <div className="flex justify-center items-center w-full">
            {/* <div className="flex items-center gap-5">
              <div className="text-5xl">📦</div>
              <div>
                <div className="text-4xl font-bold" style={{ color: 'var(--text-primary)' }}>1,234,567</div>
                <div className="text-sm mt-1.5 font-medium" style={{ color: 'var(--text-secondary)' }}>Downloads</div>
              </div>
            </div> */}
            <div className="h-16 w-px hidden sm:block"></div>
            <div className="flex items-center gap-5 justify-center">
              <div className="text-5xl">📚</div>
              <div>
                <div className="text-4xl font-bold" style={{ color: 'var(--text-primary)' }}>{packageCount}</div>
                <div className="text-sm mt-1.5 font-medium" style={{ color: 'var(--text-secondary)' }}>Packages in stock</div>
              </div>
            </div>
          </div>
        </div>
      </section>
    );
  }