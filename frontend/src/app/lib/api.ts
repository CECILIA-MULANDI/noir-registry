import { Package } from './types';

// Normalize API base URL - remove trailing slashes and semicolons
function normalizeApiUrl(url: string): string {
  return url.trim().replace(/[;\/]+$/, ''); // Remove trailing semicolons and slashes
}

// Use relative URL for client-side requests (goes through Next.js rewrite proxy)
// Use full URL for server-side requests
const rawApiUrl = typeof window === 'undefined' 
  ? (process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api')
  : '/api';
const API_BASE_URL = normalizeApiUrl(rawApiUrl);

// Debug: Log the API URL in development (helps catch configuration issues)
if (typeof window === 'undefined' && process.env.NODE_ENV === 'development') {
  console.log('[API] Raw API URL:', rawApiUrl);
  console.log('[API] Normalized API URL:', API_BASE_URL);
}

/// Function to get all packages from the API
export async function getPackages(): Promise<Package[]> {
    const url = `${API_BASE_URL}/packages`.replace(/\/+/g, '/'); // Ensure no double slashes
    
    // Debug logging (only in development)
    if (process.env.NODE_ENV === 'development') {
      console.log('[API] Fetching packages from:', url);
    }
    
    try {
      const res = await fetch(url, {
        // We can always fetch fresh data
        cache: 'no-store'
      });
      if (!res.ok) {
        // Try to get error message from response (could be JSON or text)
        let errorText = res.statusText;
        try {
          const contentType = res.headers.get('content-type');
          if (contentType?.includes('application/json')) {
            const errorJson = await res.json();
            errorText = errorJson.error || JSON.stringify(errorJson);
          } else {
            errorText = await res.text() || res.statusText;
          }
        } catch {
          // If parsing fails, use status text
          errorText = res.statusText;
        }
        console.error(`[API] Failed to fetch packages: ${res.status} ${res.statusText}`);
        console.error('[API] URL:', url);
        console.error('[API] Error details:', errorText);
        
        // Helpful error message for 404
        if (res.status === 404) {
          console.error('[API] ⚠️  404 Not Found - Is the backend server running?');
          console.error('[API] Try: curl http://localhost:8080/health');
        }
        
        return [];
      }
      return res.json();
  
    } catch (error: any) {
      // Backend might not be running - return empty array gracefully
      console.error('[API] Network error fetching packages:', error);
      console.error('[API] URL attempted:', url);
      console.error('[API] ⚠️  Is the backend server running at', API_BASE_URL.replace('/api', ''), '?');
      console.error('[API] Check: curl http://localhost:8080/health');
      return [];
    }
  }
  ///Function to search for a package
  export async function searchPackages(query: string): Promise<Package[]> {
    const url = `${API_BASE_URL}/search?q=${encodeURIComponent(query)}`.replace(/\/+/g, '/');
    try{
      const res = await fetch(url, {
        cache: 'no-store'
      })
      if (!res.ok) {
        console.error(`Failed to search packages: ${res.status} ${res.statusText}`);
        return [];
      }
      return res.json();
    }
    catch(error){
      console.warn('Error searching packages:', error);
    return [];
    }
    
  }
  export async function getPackageByName(name: string): Promise<Package | null> {
    const url = `${API_BASE_URL}/packages/${encodeURIComponent(name)}`.replace(/\/+/g, '/');
    try {
      const res = await fetch(url, {
        cache: 'no-store'
      });
      
      if (!res.ok) {
        if (res.status === 404) {
          return null; // Package not found
        }
        console.error(`Failed to fetch package: ${res.status} ${res.statusText}`);
        return null;
      }
      
      return res.json();
    } catch (error) {
      console.warn(`Error fetching package ${name}:`, error);
      return null;
    }
  }