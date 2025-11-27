import { Package } from './types';
const API_URL = 'http://localhost:8080/api';
export async function getPackages(): Promise<Package[]> {
    try {
      const res = await fetch('http://localhost:8080/api/packages', {
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
        console.error(`Failed to fetch packages: ${res.status} ${res.statusText}`);
        console.error('Error details:', errorText);
        return [];
      }
      return res.json();
  
    } catch (error) {
      // Backend might not be running - return empty array gracefully
      console.warn('Backend not available or error fetching packages:', error);
      return [];
    }
  }