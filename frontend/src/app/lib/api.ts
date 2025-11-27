import { Package } from './types';
const API_BASE_URL = 'http://localhost:8080/api';
/// Function to get all packages from the API
export async function getPackages(): Promise<Package[]> {
    try {
      const res = await fetch(`${API_BASE_URL}/packages`,{
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
  ///Function to search for a package
  export async function searchPackages(query: string): Promise<Package[]> {
    try{
      const res = await fetch(`${API_BASE_URL}/search?q=${encodeURIComponent(query)}`,{
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