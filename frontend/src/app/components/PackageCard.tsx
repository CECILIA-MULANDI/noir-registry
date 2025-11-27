import Link from 'next/link';

interface PackageCardProps {
  id: number;
  name: string;
  version: string;
}

export default function PackageCard({ id, name, version }: PackageCardProps) {
  return (
    <Link
      href={`/packages/${name}`}
      className="px-5 py-4 rounded-lg flex justify-between items-center cursor-pointer transition-all no-underline block group shadow-sm hover:shadow-md hover-card"
      style={{ 
        backgroundColor: 'var(--bg-card)', 
        border: '1px solid var(--border-color)' 
      }}
    >
      <span className="font-semibold text-sm transition-colors group-hover-text-accent" style={{ color: 'var(--text-primary)' }}>{name}</span>
      <div className="flex items-center gap-3">
        <span className="text-xs font-mono" style={{ color: 'var(--text-muted)' }}>{version}</span>
        <span className="text-lg font-bold transition-colors hover-arrow" style={{ color: 'var(--text-muted)' }}>›</span>
      </div>
    </Link>
  );
}