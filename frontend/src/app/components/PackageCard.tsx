import Link from 'next/link';

interface PackageCardProps {
  name: string;
  version: string;
  keywords?: string[];
  onKeywordClick?: (keyword: string) => void;
}

export default function PackageCard({ name, version, keywords, onKeywordClick }: PackageCardProps) {
  return (
    <Link
      href={`/packages/${name}`}
      className="px-5 py-4 rounded-lg flex justify-between items-center cursor-pointer transition-all no-underline block group shadow-sm hover:shadow-md hover-card"
      style={{
        backgroundColor: 'var(--bg-card)',
        border: '1px solid var(--border-color)'
      }}
    >
      <div className="flex flex-col gap-1.5">
        <span
          className="font-semibold text-sm transition-colors group-hover-text-accent"
          style={{ color: 'var(--text-primary)' }}
        >
          {name}
        </span>
        {keywords && keywords.length > 0 && (
          <div className="flex flex-wrap gap-1">
            {keywords.map((kw) => (
              <span
                key={kw}
                onClick={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  onKeywordClick?.(kw);
                }}
                className="px-2 py-0.5 rounded text-xs font-mono transition-opacity hover:opacity-70"
                style={{
                  backgroundColor: 'color-mix(in srgb, var(--accent-primary) 15%, transparent)',
                  color: 'var(--accent-primary)',
                  border: '1px solid color-mix(in srgb, var(--accent-primary) 35%, transparent)',
                  cursor: onKeywordClick ? 'pointer' : 'default',
                }}
              >
                {kw}
              </span>
            ))}
          </div>
        )}
      </div>
      <div className="flex items-center gap-3 flex-shrink-0 ml-4">
        <span className="text-xs font-mono" style={{ color: 'var(--text-muted)' }}>
          {version}
        </span>
        <span
          className="text-lg font-bold transition-colors hover-arrow"
          style={{ color: 'var(--text-muted)' }}
        >
          ›
        </span>
      </div>
    </Link>
  );
}
