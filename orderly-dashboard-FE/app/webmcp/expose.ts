// Exposes the WebMCP tool catalogue on `window.orderlyWebMcp` so that agents —
// in-page JS agents, browser extensions, test harnesses, userscripts — can
// discover and invoke the site's tools even when the browser does not yet
// implement the WebMCP `document.modelContext` API.
//
// This is the JS-level bridge for the "discovery cliff": without it, an agent
// whose browser lacks WebMCP sees zero tools and no hint that the page is
// agent-capable (the native registrar silently no-ops). With it, the full
// catalogue (names, descriptions, JSON Schemas) and a `call()` entry point are
// available to any same-origin script. A curl-friendly static manifest is also
// served at /webmcp.json (see routes/webmcp[.]json.ts).

import { createWebMcpTools, type WebMcpCtx } from './tools';
import { createWidgetTools } from './widget-tools';

export type AgentSurfaceTool = {
  name: string;
  title: string;
  description: string;
  inputSchema: unknown;
  annotations: { readOnlyHint?: boolean };
};

export type OrderlyAgentSurface = {
  /** Whether the browser exposes the native WebMCP `modelContext` API. */
  webmcpSupported: boolean;
  /** Number of tools exposed. */
  toolCount: number;
  /** Tool catalogue (descriptions + JSON Schemas; no execute callbacks). */
  tools: AgentSurfaceTool[];
  /** Invoke a tool by name. Rejects on unknown name or tool error. */
  call: (name: string, args?: Record<string, unknown>) => Promise<unknown>;
};

declare global {
  interface Window {
    orderlyWebMcp?: OrderlyAgentSurface;
  }
}

export function exposeAgentSurface(ctx: WebMcpCtx, widgetId?: string): void {
  if (typeof window === 'undefined') return;

  // Keep the live tool objects (with execute) in a closure; publish only the
  // static catalogue so the surface is serialisable and free of callbacks.
  const tools = widgetId ? createWidgetTools(ctx, widgetId) : createWebMcpTools(ctx);
  const catalogue: AgentSurfaceTool[] = tools.map((t) => ({
    name: t.name,
    title: t.title ?? t.name,
    description: t.description,
    inputSchema: t.inputSchema ?? { type: 'object' },
    annotations: { readOnlyHint: t.annotations?.readOnlyHint === true }
  }));

  window.orderlyWebMcp = {
    webmcpSupported:
      (typeof navigator !== 'undefined' && !!navigator.modelContext) ||
      (typeof document !== 'undefined' && !!document.modelContext),
    toolCount: tools.length,
    tools: catalogue,
    call: (name: string, args: Record<string, unknown> = {}) => {
      const tool = tools.find((t) => t.name === name);
      if (!tool) return Promise.reject(new Error(`Unknown tool: ${name}`));
      return tool.execute(args);
    }
  };
}
