import { useLocation } from 'react-router-dom'

/**
 * @reference: https://dev.to/collegewap/react-router-get-the-current-route-2e12
 * Example usage:
 *  const query = useQuery()
 *  const term = query.get("term")
 */
export function useQueryParams() {
  // Use the URLSearchParams API to extract the query parameters
  // useLocation().search will have the query parameters eg: ?foo=bar&a=b
  return new URLSearchParams(useLocation().search)
}
