import { FC, useEffect, useRef, useState } from 'react';

type SearchResult = {
  id: string;
  label: string;
  category: string;
  description?: string;
};

const ALL_RESULTS: SearchResult[] = [
  {
    id: 'dashboards',
    label: 'Dashboards',
    category: 'Navigation',
    description: 'Overview of key metrics'
  },
  {
    id: 'leaderboard',
    label: 'Leaderboard',
    category: 'Navigation',
    description: 'Trading performance and open positions'
  },
  {
    id: 'explorer',
    label: 'Explorer',
    category: 'Navigation',
    description: 'Search wallets and accounts'
  },
  {
    id: 'btc-perp',
    label: 'BTC-PERP',
    category: 'Trading Pair',
    description: '$67,234 · Volume $145M'
  },
  {
    id: 'eth-perp',
    label: 'ETH-PERP',
    category: 'Trading Pair',
    description: '$3,521 · Volume $98M'
  },
  {
    id: 'sol-perp',
    label: 'SOL-PERP',
    category: 'Trading Pair',
    description: '$178 · Volume $52M'
  },
  {
    id: 'arb-perp',
    label: 'ARB-PERP',
    category: 'Trading Pair',
    description: '$1.24 · Volume $31M'
  },
  {
    id: 'woofi',
    label: 'WOOFi',
    category: 'Broker',
    description: 'Volume $125M · Market Share 29.4%'
  },
  {
    id: 'orderly-dex',
    label: 'Orderly DEX',
    category: 'Broker',
    description: 'Volume $98M · Market Share 23%'
  },
  {
    id: 'aeterna',
    label: 'Aeterna',
    category: 'Broker',
    description: 'Volume $72M · Market Share 16.9%'
  },
  {
    id: 'volume-7d',
    label: '7D Volume',
    category: 'Metric',
    description: 'Total trading volume last 7 days'
  },
  {
    id: 'active-traders',
    label: 'Active Traders',
    category: 'Metric',
    description: '24h unique active traders count'
  },
  {
    id: 'open-interest',
    label: 'Open Interest',
    category: 'Metric',
    description: 'Total open interest across all pairs'
  }
];

const CATEGORY_ORDER = ['Navigation', 'Trading Pair', 'Broker', 'Metric'];

type SearchModalProps = {
  open: boolean;
  onClose: () => void;
  onNavigate?: (id: string) => void;
};

export const SearchModal: FC<SearchModalProps> = ({ open, onClose, onNavigate }) => {
  const [query, setQuery] = useState('');
  const [selectedIdx, setSelectedIdx] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const filtered = query.trim()
    ? ALL_RESULTS.filter(
        (r) =>
          r.label.toLowerCase().includes(query.toLowerCase()) ||
          r.description?.toLowerCase().includes(query.toLowerCase()) ||
          r.category.toLowerCase().includes(query.toLowerCase())
      )
    : ALL_RESULTS;

  const grouped: Record<string, SearchResult[]> = {};
  CATEGORY_ORDER.forEach((cat) => {
    const items = filtered.filter((r) => r.category === cat);
    if (items.length > 0) grouped[cat] = items;
  });

  const flatFiltered = CATEGORY_ORDER.flatMap((cat) => grouped[cat] ?? []);

  useEffect(() => {
    setSelectedIdx(0);
  }, [query]);

  useEffect(() => {
    if (open) {
      setQuery('');
      setSelectedIdx(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [open]);

  useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (!open) return;
      if (e.key === 'Escape') {
        onClose();
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        setSelectedIdx((i) => Math.min(i + 1, flatFiltered.length - 1));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        setSelectedIdx((i) => Math.max(i - 1, 0));
      } else if (e.key === 'Enter') {
        const item = flatFiltered[selectedIdx];
        if (item) handleSelect(item);
      }
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [open, flatFiltered, selectedIdx]); // eslint-disable-line react-hooks/exhaustive-deps

  const handleSelect = (item: SearchResult) => {
    onNavigate?.(item.id);
    onClose();
  };

  if (!open) return null;

  let flatIdx = 0;

  return (
    <div
      style={{
        position: 'fixed',
        inset: 0,
        zIndex: 200,
        display: 'flex',
        alignItems: 'flex-start',
        justifyContent: 'center',
        paddingTop: 80
      }}
    >
      {/* Backdrop */}
      <div
        role="button"
        tabIndex={-1}
        onClick={onClose}
        onKeyDown={(e) => {
          if (e.key === 'Escape') onClose();
        }}
        style={{
          position: 'absolute',
          inset: 0,
          background: 'rgba(0,0,0,0.7)',
          backdropFilter: 'blur(4px)'
        }}
      />

      {/* Modal */}
      <div
        style={{
          position: 'relative',
          width: '100%',
          maxWidth: 560,
          background: 'rgba(16,8,28,0.98)',
          border: '1px solid rgba(156,117,255,0.25)',
          borderRadius: 16,
          boxShadow: '0 24px 80px rgba(0,0,0,0.6), 0 0 0 1px rgba(156,117,255,0.08)',
          overflow: 'hidden',
          margin: '0 16px'
        }}
      >
        {/* Search input */}
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 10,
            padding: '14px 16px',
            borderBottom: '1px solid rgba(156,117,255,0.12)'
          }}
        >
          <svg
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="rgba(255,255,255,0.4)"
            strokeWidth="2"
          >
            <circle cx="11" cy="11" r="8" />
            <line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search metrics, pairs, brokers…"
            style={{
              flex: 1,
              background: 'none',
              border: 'none',
              outline: 'none',
              color: '#fff',
              fontSize: 15,
              fontWeight: 400,
              caretColor: '#9C75FF'
            }}
          />
          <kbd
            role="button"
            tabIndex={0}
            style={{
              background: 'rgba(255,255,255,0.06)',
              border: '1px solid rgba(255,255,255,0.12)',
              borderRadius: 6,
              padding: '2px 7px',
              fontSize: 11,
              color: 'rgba(255,255,255,0.35)',
              fontFamily: 'monospace',
              cursor: 'pointer'
            }}
            onClick={onClose}
            onKeyDown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') onClose();
            }}
          >
            ESC
          </kbd>
        </div>

        {/* Results */}
        <div style={{ maxHeight: 380, overflowY: 'auto', padding: '8px 0' }}>
          {flatFiltered.length === 0 ? (
            <div
              style={{
                padding: '32px 20px',
                textAlign: 'center',
                color: 'rgba(255,255,255,0.3)',
                fontSize: 14
              }}
            >
              No results for &ldquo;{query}&rdquo;
            </div>
          ) : (
            CATEGORY_ORDER.map((cat) => {
              const items = grouped[cat];
              if (!items) return null;
              return (
                <div key={cat}>
                  <div
                    style={{
                      padding: '6px 16px 4px',
                      fontSize: 10,
                      fontWeight: 600,
                      color: 'rgba(156,117,255,0.6)',
                      textTransform: 'uppercase',
                      letterSpacing: '0.09em'
                    }}
                  >
                    {cat}
                  </div>
                  {items.map((item) => {
                    const idx = flatIdx++;
                    const isSelected = selectedIdx === idx;
                    return (
                      <div
                        key={item.id}
                        role="button"
                        tabIndex={0}
                        onClick={() => handleSelect(item)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') handleSelect(item);
                        }}
                        onMouseEnter={() => setSelectedIdx(idx)}
                        style={{
                          display: 'flex',
                          alignItems: 'center',
                          gap: 12,
                          padding: '9px 16px',
                          cursor: 'pointer',
                          background: isSelected ? 'rgba(156,117,255,0.12)' : 'transparent',
                          borderLeft: isSelected ? '2px solid #9C75FF' : '2px solid transparent',
                          transition: 'background 0.1s'
                        }}
                      >
                        <div
                          style={{
                            width: 32,
                            height: 32,
                            borderRadius: 8,
                            background: 'rgba(156,117,255,0.1)',
                            border: '1px solid rgba(156,117,255,0.2)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            flexShrink: 0,
                            fontSize: 12,
                            color: '#9C75FF',
                            fontWeight: 700
                          }}
                        >
                          {item.label.slice(0, 2).toUpperCase()}
                        </div>
                        <div style={{ minWidth: 0 }}>
                          <div
                            style={{
                              fontSize: 13,
                              fontWeight: 500,
                              color: isSelected ? '#fff' : 'rgba(255,255,255,0.8)',
                              lineHeight: 1.3
                            }}
                          >
                            {item.label}
                          </div>
                          {item.description && (
                            <div
                              style={{
                                fontSize: 11,
                                color: 'rgba(255,255,255,0.35)',
                                marginTop: 1,
                                overflow: 'hidden',
                                textOverflow: 'ellipsis',
                                whiteSpace: 'nowrap'
                              }}
                            >
                              {item.description}
                            </div>
                          )}
                        </div>
                      </div>
                    );
                  })}
                </div>
              );
            })
          )}
        </div>

        {/* Footer hint */}
        <div
          style={{
            padding: '8px 16px',
            borderTop: '1px solid rgba(156,117,255,0.1)',
            display: 'flex',
            gap: 16,
            fontSize: 11,
            color: 'rgba(255,255,255,0.25)'
          }}
        >
          <span>
            <kbd style={{ fontFamily: 'monospace', fontSize: 10 }}>↑↓</kbd> navigate
          </span>
          <span>
            <kbd style={{ fontFamily: 'monospace', fontSize: 10 }}>↵</kbd> select
          </span>
          <span>
            <kbd style={{ fontFamily: 'monospace', fontSize: 10 }}>esc</kbd> close
          </span>
        </div>
      </div>
    </div>
  );
};
