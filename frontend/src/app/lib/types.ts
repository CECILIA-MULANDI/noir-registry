export  interface Package {
    id: number;
    name: string;
    description: string | null;
    latest_version: string | null;
    github_stars: number | null;
    created_at: string;
    updated_at: string;
  }