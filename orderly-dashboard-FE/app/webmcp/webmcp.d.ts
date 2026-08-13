// Ambient types for the WebMCP browser API (https://webmachinelearning.github.io/webmcp/).
// Allows a page to register imperative tools that an in-page AI agent can call.

interface ModelContextToolAnnotations {
  readOnlyHint?: boolean;
  untrustedContentHint?: boolean;
}

interface ModelContextTool {
  name: string;
  title?: string;
  description: string;
  inputSchema?: object;
  execute: (args: Record<string, unknown>) => Promise<unknown>;
  annotations?: ModelContextToolAnnotations;
}

interface ModelContextRegisterToolOptions {
  signal?: AbortSignal;
  exposedTo?: string[];
}

interface ModelContextGetToolsOptions {
  signal?: AbortSignal;
  exposedTo?: string[];
}

interface ModelContext {
  registerTool(tool: ModelContextTool, options?: ModelContextRegisterToolOptions): Promise<void>;
  getTools(options?: ModelContextGetToolsOptions): Promise<unknown[]>;
}

interface Navigator {
  readonly modelContext?: ModelContext;
}

interface Document {
  readonly modelContext?: ModelContext;
}
