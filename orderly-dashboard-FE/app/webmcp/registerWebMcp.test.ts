import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const fakeTool = (name: string): ModelContextTool => ({
  name,
  title: name,
  description: 'd',
  inputSchema: { type: 'object', properties: {}, additionalProperties: false },
  annotations: { readOnlyHint: true },
  execute: vi.fn()
});

function setModelContext(mc: unknown) {
  (navigator as unknown as { modelContext: unknown }).modelContext = mc;
}

function clearModelContext() {
  delete (navigator as unknown as { modelContext?: unknown }).modelContext;
  delete (document as unknown as { modelContext?: unknown }).modelContext;
}

describe('registerWebMcpTools', () => {
  beforeEach(() => {
    vi.resetModules();
    clearModelContext();
  });

  afterEach(() => {
    vi.resetModules();
    clearModelContext();
  });

  it('is a no-op when no modelContext is available', async () => {
    const registerTool = vi.fn();
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('a')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: vi.fn(() => []) }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    expect(registerTool).not.toHaveBeenCalled();
  });

  it('registers every tool when modelContext is present', async () => {
    const registerTool = vi.fn().mockResolvedValue(undefined);
    setModelContext({ registerTool });
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('a'), fakeTool('b'), fakeTool('c')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: vi.fn(() => []) }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    expect(registerTool).toHaveBeenCalledTimes(3);
    expect(registerTool.mock.calls[0][1]).toEqual({ signal: expect.any(AbortSignal) });
  });

  it('registers only the widget tool set when a widgetId is given', async () => {
    const registerTool = vi.fn().mockResolvedValue(undefined);
    setModelContext({ registerTool });
    const widgetFactory = vi.fn(() => [fakeTool('only-widget')]);
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('full')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: widgetFactory }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' }, 'volume');
    expect(widgetFactory).toHaveBeenCalledWith({ evmApiUrl: 'u', queryServiceUrl: 'q' }, 'volume');
    expect(registerTool).toHaveBeenCalledTimes(1);
    expect(registerTool.mock.calls[0][0].name).toBe('only-widget');
  });

  it('aborts the previous registration on re-entry', async () => {
    const registerTool = vi.fn().mockResolvedValue(undefined);
    setModelContext({ registerTool });
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('a'), fakeTool('b')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: vi.fn(() => []) }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    const firstSignal = registerTool.mock.calls[0][1].signal as AbortSignal;
    expect(firstSignal.aborted).toBe(false);
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    expect(firstSignal.aborted).toBe(true);
  });

  it('aborts on pagehide', async () => {
    const registerTool = vi.fn().mockResolvedValue(undefined);
    setModelContext({ registerTool });
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('a')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: vi.fn(() => []) }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    const signal = registerTool.mock.calls[0][1].signal as AbortSignal;
    expect(signal.aborted).toBe(false);
    window.dispatchEvent(new Event('pagehide'));
    expect(signal.aborted).toBe(true);
  });

  it('continues registering (and warns) after a non-aborting rejection', async () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const registerTool = vi.fn().mockRejectedValue(new Error('NotAllowedError'));
    setModelContext({ registerTool });
    vi.doMock('./tools', () => ({
      createWebMcpTools: vi.fn(() => [fakeTool('a'), fakeTool('b'), fakeTool('c')]),
      WebMcpCtx: {}
    }));
    vi.doMock('./widget-tools', () => ({ createWidgetTools: vi.fn(() => []) }));
    const { registerWebMcpTools } = await import('./registerWebMcp');
    await registerWebMcpTools({ evmApiUrl: 'u', queryServiceUrl: 'q' });
    expect(registerTool).toHaveBeenCalledTimes(3);
    expect(warn).toHaveBeenCalledTimes(3);
  });
});
