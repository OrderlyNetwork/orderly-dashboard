// Registers WebMCP tools with the browser's modelContext API.
// Mirrors the dex-creator pattern (commit 9673be1): one AbortController owns every
// registration; aborting unregisters all tools. Feature-detected — no-op when the
// browser has no modelContext (i.e. no in-page agent).

import { createWebMcpTools, type WebMcpCtx } from './tools';
import { createWidgetTools } from './widget-tools';

let activeController: AbortController | null = null;

function getModelContext(): ModelContext | undefined {
  if (typeof navigator !== 'undefined' && navigator.modelContext) return navigator.modelContext;
  if (typeof document !== 'undefined' && document.modelContext) return document.modelContext;
  return undefined;
}

export async function registerWebMcpTools(ctx: WebMcpCtx, widgetId?: string): Promise<void> {
  const modelContext = getModelContext();
  if (!modelContext) return;

  // Abort any previous registration (HMR / re-entry).
  if (activeController) {
    activeController.abort();
    activeController = null;
  }

  const controller = new AbortController();
  activeController = controller;

  const tools = widgetId ? createWidgetTools(ctx, widgetId) : createWebMcpTools(ctx);
  for (const tool of tools) {
    try {
      await modelContext.registerTool(tool, { signal: controller.signal });
    } catch (err) {
      // Aborted (HMR/re-entry) or blocked by the "tools" permissions policy on a
      // cross-origin embed: stop registering the remaining tools when aborted.
      if (controller.signal.aborted) break;
      // A genuinely broken/rejected tool definition must not vanish silently.
      console.warn(`[webmcp] tool "${tool.name}" was rejected by modelContext:`, err);
    }
  }

  // Unregister everything when the page is torn down.
  window.addEventListener(
    'pagehide',
    () => {
      if (activeController === controller) {
        controller.abort();
        activeController = null;
      }
    },
    { once: true }
  );
}
