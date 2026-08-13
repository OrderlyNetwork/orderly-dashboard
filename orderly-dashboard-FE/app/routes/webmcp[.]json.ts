// Static WebMCP tool manifest served at /webmcp.json.
//
// Lets non-JS / scripted agents (curl, fetch, crawlers) discover the site's
// WebMCP tool catalogue — names, titles, descriptions, and JSON Schemas —
// without running the page or needing a browser that implements
// `document.modelContext`. The native modelContext registration and the
// runtime `window.orderlyWebMcp` surface (see webmcp/expose.ts) remain the
// primary paths for in-browser agents; this manifest is the crawlable beacon.

import { json } from '@remix-run/node';

import { createWebMcpTools } from '~/webmcp/tools';

export function loader() {
  // Tool definitions (name/title/description/schema) are static; execute
  // callbacks need a runtime ctx, so they are stripped from the manifest.
  const tools = createWebMcpTools({ evmApiUrl: '', queryServiceUrl: '' });
  const catalogue = tools.map(({ name, title, description, inputSchema, annotations }) => ({
    name,
    title,
    description,
    inputSchema,
    annotations
  }));

  return json(
    {
      site: 'Orderly Dashboard',
      spec: 'WebMCP (https://webmachinelearning.github.io/webmcp/)',
      webmcpSupported: true,
      note:
        'This site exposes WebMCP tools. Browsers implementing document.modelContext ' +
        'discover them automatically; others can read this manifest or use ' +
        'window.orderlyWebMcp at runtime to list and invoke tools.',
      toolCount: catalogue.length,
      tools: catalogue
    },
    { headers: { 'Cache-Control': 'public, max-age=300' } }
  );
}
