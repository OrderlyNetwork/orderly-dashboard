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
