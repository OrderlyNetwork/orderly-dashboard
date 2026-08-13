import { useNavigate } from '@remix-run/react';
import { useEffect } from 'react';

// Bridges a plain-DOM `orderly:nav` CustomEvent to the Remix router. Nav tools
// (app/webmcp/nav-tools.ts) live in plain TS and cannot call `useNavigate`
// directly, so they dispatch `window.dispatchEvent(new CustomEvent('orderly:nav',
// { detail: { path } }))` and this component performs the client-side navigate.
// This preserves shared SPA context (no full reload).
export function NavBridge() {
  const navigate = useNavigate();
  useEffect(() => {
    const onNav = (e: Event) => {
      const path = (e as CustomEvent<{ path: string }>).detail?.path;
      if (typeof path === 'string' && path.startsWith('/')) navigate(path);
    };
    window.addEventListener('orderly:nav', onNav as EventListener);
    return () => window.removeEventListener('orderly:nav', onNav as EventListener);
  }, [navigate]);
  return null;
}
