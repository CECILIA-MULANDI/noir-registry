interface PackageCardProps {
    name: string;
    version: string;

}
export default function PackageCard({ name, version }: PackageCardProps) {
    return (
        <a href={`/packages/${name}`}
            className="bg-[var(--bg-card)] border border-[var(--border-color)] px-4 py-3 mb-2 rounded flex justify-between items-center cursor-pointer hover:bg-[var(--bg-card-hover)] transition-colors no-underline">
            <span className="text-[var(--text-primary)] font-medium text-sm">{name}</span>
            <div className="flex items-center gap-2">
                <span className="text-[var(--text-muted)] text-xs">{version}</span>
                <span className="text-[var(--text-muted)]">›</span>
            </div>
        </a>
    )
}