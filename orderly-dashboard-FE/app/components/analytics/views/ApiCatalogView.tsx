import { FC, useState } from 'react';

type HttpMethod = 'GET' | 'POST' | 'DELETE' | 'PUT';

type Endpoint = {
  method: HttpMethod;
  path: string;
  description: string;
  category: string;
  version: string;
  auth: boolean;
};

const METHOD_COLORS: Record<HttpMethod, { bg: string; text: string }> = {
  GET: { bg: 'rgba(52,211,153,0.15)', text: '#34D399' },
  POST: { bg: 'rgba(251,146,60,0.15)', text: '#FB923C' },
  PUT: { bg: 'rgba(96,165,250,0.15)', text: '#60A5FA' },
  DELETE: { bg: 'rgba(248,113,113,0.15)', text: '#F87171' }
};

const ENDPOINTS: Endpoint[] = [
  // Market Data
  { method: 'GET', path: '/v1/public/info', description: 'Get exchange info and supported assets', category: 'Market Data', version: 'v1', auth: false },
  { method: 'GET', path: '/v1/public/market_trades', description: 'Get recent public market trades for a symbol', category: 'Market Data', version: 'v1', auth: false },
  { method: 'GET', path: '/v1/public/orderbook/{symbol}', description: 'Get current order book snapshot', category: 'Market Data', version: 'v1', auth: false },
  { method: 'GET', path: '/v1/public/kline', description: 'Get OHLCV candlestick data', category: 'Market Data', version: 'v1', auth: false },
  { method: 'GET', path: '/v1/public/funding_rate/{symbol}', description: 'Get current and predicted funding rate', category: 'Market Data', version: 'v1', auth: false },
  // Account
  { method: 'GET', path: '/v1/client/info', description: 'Get account info and balances', category: 'Account', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/client/holding', description: 'Get current positions and holdings', category: 'Account', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/asset/history', description: 'Get deposit and withdrawal history', category: 'Account', version: 'v1', auth: true },
  // Orders
  { method: 'POST', path: '/v1/order', description: 'Place a new order', category: 'Orders', version: 'v1', auth: true },
  { method: 'DELETE', path: '/v1/order', description: 'Cancel an existing order by order_id', category: 'Orders', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/order/{order_id}', description: 'Get details of a specific order', category: 'Orders', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/orders', description: 'Get list of orders with filters', category: 'Orders', version: 'v1', auth: true },
  { method: 'POST', path: '/v1/batch-order', description: 'Place multiple orders in a single call', category: 'Orders', version: 'v1', auth: true },
  // Trades
  { method: 'GET', path: '/v1/client/trades', description: 'Get historical fills for the account', category: 'Trades', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/client/trades/{trade_id}', description: 'Get a specific trade by ID', category: 'Trades', version: 'v1', auth: true },
  // Analytics
  { method: 'GET', path: '/v1/client/statistics', description: 'Get aggregated PnL and volume statistics', category: 'Analytics', version: 'v1', auth: true },
  { method: 'GET', path: '/v1/public/stats', description: 'Get protocol-level stats (volume, users, fees)', category: 'Analytics', version: 'v1', auth: false },
];

const CATEGORIES = ['All', ...Array.from(new Set(ENDPOINTS.map((e) => e.category)))];

export const ApiCatalogView: FC = () => {
  const [activeCategory, setActiveCategory] = useState('All');
  const [search, setSearch] = useState('');
  const [expanded, setExpanded] = useState<string | null>(null);

  const filtered = ENDPOINTS.filter((e) => {
    const matchCat = activeCategory === 'All' || e.category === activeCategory;
    const matchSearch =
      !search ||
      e.path.toLowerCase().includes(search.toLowerCase()) ||
      e.description.toLowerCase().includes(search.toLowerCase());
    return matchCat && matchSearch;
  });

  const grouped: Record<string, Endpoint[]> = {};
  filtered.forEach((e) => {
    if (!grouped[e.category]) grouped[e.category] = [];
    grouped[e.category].push(e);
  });

  return (
    <div style={{ maxWidth: 900 }}>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 6 }}>
          <div
            style={{
              width: 36,
              height: 36,
              borderRadius: 10,
              background: 'rgba(251,146,60,0.15)',
              border: '1px solid rgba(251,146,60,0.3)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#FB923C'
            }}
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="16 18 22 12 16 6" />
              <polyline points="8 6 2 12 8 18" />
            </svg>
          </div>
          <div>
            <h1 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: '#fff' }}>API Catalog</h1>
            <p style={{ margin: 0, fontSize: 13, color: 'rgba(255,255,255,0.45)' }}>
              Orderly Network REST API — {ENDPOINTS.length} endpoints
            </p>
          </div>
        </div>
      </div>

      {/* Search + filters */}
      <div style={{ display: 'flex', gap: 12, marginBottom: 20, flexWrap: 'wrap' }}>
        <div style={{ position: 'relative', flex: 1, minWidth: 200 }}>
          <svg
            width="14" height="14"
            viewBox="0 0 24 24" fill="none" stroke="rgba(255,255,255,0.3)" strokeWidth="2"
            style={{ position: 'absolute', left: 12, top: '50%', transform: 'translateY(-50%)' }}
          >
            <circle cx="11" cy="11" r="8" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search endpoints..."
            style={{
              width: '100%',
              paddingLeft: 34,
              paddingRight: 12,
              height: 36,
              borderRadius: 8,
              border: '1px solid rgba(255,255,255,0.1)',
              background: 'rgba(255,255,255,0.05)',
              color: '#fff',
              fontSize: 13,
              outline: 'none',
              boxSizing: 'border-box'
            }}
          />
        </div>
        <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap' }}>
          {CATEGORIES.map((cat) => (
            <button
              key={cat}
              onClick={() => setActiveCategory(cat)}
              style={{
                padding: '0 14px',
                height: 36,
                borderRadius: 8,
                border: activeCategory === cat ? '1px solid rgba(251,146,60,0.5)' : '1px solid rgba(255,255,255,0.1)',
                background: activeCategory === cat ? 'rgba(251,146,60,0.15)' : 'rgba(255,255,255,0.04)',
                color: activeCategory === cat ? '#FB923C' : 'rgba(255,255,255,0.5)',
                fontSize: 12,
                fontWeight: activeCategory === cat ? 600 : 400,
                cursor: 'pointer',
                transition: 'all 0.15s',
                whiteSpace: 'nowrap'
              }}
            >
              {cat}
            </button>
          ))}
        </div>
      </div>

      {/* Endpoint groups */}
      {Object.entries(grouped).map(([category, endpoints]) => (
        <div key={category} style={{ marginBottom: 28 }}>
          <div
            style={{
              fontSize: 11,
              fontWeight: 600,
              color: 'rgba(255,255,255,0.3)',
              letterSpacing: '0.08em',
              textTransform: 'uppercase',
              marginBottom: 8,
              paddingLeft: 2
            }}
          >
            {category}
          </div>
          <div
            style={{
              border: '1px solid rgba(255,255,255,0.07)',
              borderRadius: 12,
              overflow: 'hidden'
            }}
          >
            {endpoints.map((ep, i) => {
              const key = `${ep.method}:${ep.path}`;
              const isOpen = expanded === key;
              const mc = METHOD_COLORS[ep.method];
              return (
                <div key={key}>
                  {i > 0 && (
                    <div style={{ height: 1, background: 'rgba(255,255,255,0.05)' }} />
                  )}
                  <button
                    onClick={() => setExpanded(isOpen ? null : key)}
                    style={{
                      width: '100%',
                      display: 'flex',
                      alignItems: 'center',
                      gap: 12,
                      padding: '12px 16px',
                      border: 'none',
                      background: isOpen ? 'rgba(255,255,255,0.04)' : 'transparent',
                      cursor: 'pointer',
                      textAlign: 'left',
                      transition: 'background 0.12s'
                    }}
                  >
                    {/* Method badge */}
                    <span
                      style={{
                        flexShrink: 0,
                        padding: '2px 8px',
                        borderRadius: 5,
                        fontSize: 11,
                        fontWeight: 700,
                        letterSpacing: '0.04em',
                        background: mc.bg,
                        color: mc.text,
                        minWidth: 52,
                        textAlign: 'center'
                      }}
                    >
                      {ep.method}
                    </span>
                    {/* Path */}
                    <span
                      style={{
                        fontFamily: 'monospace',
                        fontSize: 13,
                        color: '#fff',
                        fontWeight: 500
                      }}
                    >
                      {ep.path}
                    </span>
                    {/* Auth badge */}
                    {ep.auth && (
                      <span
                        style={{
                          marginLeft: 'auto',
                          flexShrink: 0,
                          fontSize: 10,
                          padding: '2px 7px',
                          borderRadius: 4,
                          background: 'rgba(156,117,255,0.12)',
                          color: 'rgba(156,117,255,0.8)',
                          border: '1px solid rgba(156,117,255,0.2)',
                          fontWeight: 600
                        }}
                      >
                        AUTH
                      </span>
                    )}
                    {/* Chevron */}
                    <svg
                      width="14" height="14" viewBox="0 0 24 24" fill="none"
                      stroke="rgba(255,255,255,0.3)" strokeWidth="2"
                      style={{
                        flexShrink: 0,
                        marginLeft: ep.auth ? 8 : 'auto',
                        transform: isOpen ? 'rotate(180deg)' : 'none',
                        transition: 'transform 0.15s'
                      }}
                    >
                      <polyline points="6 9 12 15 18 9" />
                    </svg>
                  </button>
                  {/* Expanded detail */}
                  {isOpen && (
                    <div
                      style={{
                        padding: '0 16px 16px 88px',
                        background: 'rgba(255,255,255,0.02)'
                      }}
                    >
                      <p style={{ margin: '0 0 12px', fontSize: 13, color: 'rgba(255,255,255,0.6)', lineHeight: 1.6 }}>
                        {ep.description}
                      </p>
                      <div style={{ display: 'flex', gap: 8 }}>
                        <span style={{ fontSize: 11, padding: '2px 8px', borderRadius: 4, background: 'rgba(255,255,255,0.06)', color: 'rgba(255,255,255,0.4)' }}>
                          {ep.version}
                        </span>
                        <span style={{ fontSize: 11, padding: '2px 8px', borderRadius: 4, background: 'rgba(255,255,255,0.06)', color: 'rgba(255,255,255,0.4)' }}>
                          REST
                        </span>
                        <span style={{ fontSize: 11, padding: '2px 8px', borderRadius: 4, background: 'rgba(255,255,255,0.06)', color: 'rgba(255,255,255,0.4)' }}>
                          {ep.auth ? 'Requires API key' : 'Public'}
                        </span>
                      </div>
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      ))}

      {filtered.length === 0 && (
        <div style={{ textAlign: 'center', padding: '60px 0', color: 'rgba(255,255,255,0.25)', fontSize: 14 }}>
          No endpoints match your search.
        </div>
      )}
    </div>
  );
};
