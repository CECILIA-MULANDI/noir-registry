interface StatsSectionProps {
    packageCount: number;
  }
  
  export default function StatsSection({ packageCount }: StatsSectionProps) {
    return (
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
                <div className="text-4xl font-bold" style={{ color: 'var(--text-primary)' }}>{packageCount}</div>
                <div className="text-sm mt-1.5 font-medium" style={{ color: 'var(--text-secondary)' }}>Packages in stock</div>
              </div>
            </div>
          </div>
        </div>
      </section>
    );
  }