import { useEffect } from 'react';

import { useAppState } from '~/App';
import { exposeAgentSurface } from '~/webmcp/expose';
import { registerWebMcpTools } from '~/webmcp/registerWebMcp';

// Mounts inside AppContext.Provider. Resolves the EVM/Query base URLs from
// AppContext (populated by root.tsx's loader) and registers WebMCP tools once.
// No-op in browsers without a modelContext (no in-page agent present). Pass a
// `widgetId` to register only the tool(s) for that embeddable widget.
export function WebMcpRegistrar({ widgetId }: { widgetId?: string } = {}) {
  const { queryServiceUrl, evmApiUrl } = useAppState();

  useEffect(() => {
    if (!evmApiUrl || !queryServiceUrl) return;
    // Always publish the catalogue for pre-WebMCP / in-page agents (discovery
    // cliff), then register with the native modelContext API when present.
    exposeAgentSurface({ evmApiUrl, queryServiceUrl }, widgetId);
    void registerWebMcpTools({ evmApiUrl, queryServiceUrl }, widgetId);
  }, [evmApiUrl, queryServiceUrl, widgetId]);

  return null;
}
