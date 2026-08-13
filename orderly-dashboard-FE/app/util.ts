export function base64UrlSafeEncode(input: string) {
  const base64 = btoa(input);
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

export function base64UrlSafeDecode(input: string) {
  const base64 = input.replace(/-/g, '+').replace(/_/g, '/');
  // Pad with '=' to make the length a multiple of 4
  const padding = base64.length % 4 === 0 ? '' : '='.repeat(4 - (base64.length % 4));
  return atob(base64 + padding);
}

export const DASHBOARD_ORIGIN = 'https://dashboard.orderly.network';

// Solana addresses are 43-44 chars of alphanumeric (base58 alphabet minus 0/O/I/l
// is a subset of [0-9a-zA-Z], so this shape check is sufficient to distinguish a
// raw Solana address from an EVM hex or Orderly account_id).
export const SOL_REGEX = /^[0-9a-zA-Z]{43,44}$/;

// Orderly address URLs must carry Solana addresses as url-safe base64 (they
// contain characters illegal in a path segment). EVM and account_id values are
// already URL-safe and pass through unchanged. Mirrors decode in address route.
export const encodeAddress = (address: string) =>
  address.match(SOL_REGEX) ? base64UrlSafeEncode(address) : address;
