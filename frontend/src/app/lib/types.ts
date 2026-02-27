export interface Package {
  id: number;
  name: string;
  description: string | null;
  latest_version: string | null;
  github_stars: number | null;
  created_at: string;
  updated_at: string;
  github_repository_url?: string;
  homepage?: string | null;
  license?: string | null;
  owner_github_username?: string;
  owner_avatar_url?: string | null;
  total_downloads?: number;
  keywords?: string[];
}

export interface Category {
  id: number;
  name: string;
  slug: string;
  description: string | null;
}