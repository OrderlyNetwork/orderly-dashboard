import { FC, useState } from 'react';

type StarFn = (item: {
  id: string;
  type: 'dashboard' | 'query';
  title: string;
  description: string;
}) => void;

type Query = {
  id: string;
  name: string;
  description: string;
  category: 'Volume' | 'Fees' | 'Traders' | 'Pairs' | 'Custom';
  lastRun: string;
  runCount: number;
  saved: boolean;
};

const MOCK_QUERIES: Query[] = [
  {
    id: 'q1',
    name: 'Daily Volume by Broker',
    description: 'Aggregate trading volume grouped by broker_id for the last 30 days',
    category: 'Volume',
    lastRun: '2m ago',
    runCount: 142,
    saved: true
  },
  {
    id: 'q2',
    name: 'Top 100 Traders by PnL',
    description: 'Ranked list of traders by realized PnL over the selected date range',
    category: 'Traders',
    lastRun: '15m ago',
    runCount: 89,
    saved: true
  },
  {
    id: 'q3',
    name: 'Fee Revenue by Tier',
    description: 'Breakdown of fee income per fee tier with user counts',
    category: 'Fees',
    lastRun: '1h ago',
    runCount: 34,
    saved: true
  },
  {
    id: 'q4',
    name: 'Perp Pair Dominance',
    description: 'Market share of each trading pair by volume over time',
    category: 'Pairs',
    lastRun: '3h ago',
    runCount: 67,
    saved: false
  },
  {
    id: 'q5',
    name: 'Liquidation Heatmap',
    description: 'Liquidation events by hour of day and day of week',
    category: 'Traders',
    lastRun: '6h ago',
    runCount: 21,
    saved: true
  },
  {
    id: 'q6',
    name: 'Funding Rate Arbitrage',
    description: 'Cross-pair funding rate differentials over 90 day window',
    category: 'Pairs',
    lastRun: '1d ago',
    runCount: 12,
    saved: false
  },
  {
    id: 'q7',
    name: 'Broker Retention Rate',
    description: 'Returning trader percentage per broker month-over-month',
    category: 'Volume',
    lastRun: '2d ago',
    runCount: 8,
    saved: true
  },
  {
    id: 'q8',
    name: 'Custom Query',
    description: 'Ad-hoc SQL workspace for live data exploration',
    category: 'Custom',
    lastRun: '5m ago',
    runCount: 204,
    saved: false
  }
];

const CATEGORY_COLORS: Record<Query['category'], string> = {
  Volume: '#9C75FF',
  Fees: '#f59e0b',
  Traders: '#34d399',
  Pairs: '#60a5fa',
  Custom: '#f87171'
};

const MOCK_RESULT = `| broker_id   | date       | volume_usd   | trades |
|-------------|------------|--------------|--------|
| woofi       | 2026-04-21 | 125,342,100  | 8,432  |
| orderly     | 2026-04-21 | 98,021,400   | 6,218  |
| aeterna     | 2026-04-21 | 72,450,800   | 4,931  |
| flowx       | 2026-04-21 | 55,100,200   | 3,102  |
| perppro     | 2026-04-21 | 38,220,000   | 2,455  |`;

type Props = { isStarred: (id: string) => boolean; toggleStar: StarFn };

export const QueriesView: FC<Props> = ({ isStarred, toggleStar }) => {
  const [activeQuery, setActiveQuery] = useState<Query | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [hasResult, setHasResult] = useState(false);
  const [filter, setFilter] = useState('');

  const filtered = filter
    ? MOCK_QUERIES.filter(
        (q) =>
          q.name.toLowerCase().includes(filter.toLowerCase()) ||
          q.category.toLowerCase().includes(filter.toLowerCase())
      )
    : MOCK_QUERIES;

  const handleRun = () => {
    if (!activeQuery) return;
    setIsRunning(true);
    setHasResult(false);
    setTimeout(() => {
      setIsRunning(false);
      setHasResult(true);
    }, 1200);
  };

  return (
    <div style={{ display: 'flex', gap: 16, height: '100%', minHeight: 500 }}>
      {/* Query list sidebar */}
      <div
        style={{
          width: 300,
          flexShrink: 0,
          background: 'rgba(8,20,16,.95)',
          border: '1px solid rgba(52,211,153,0.18)',
          borderRadius: 16,
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden'
        }}
      >
        <div style={{ padding: '14px 16px', borderBottom: '1px solid rgba(52,211,153,0.12)' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
            <svg
              width="14"
              height="14"
              viewBox="0 0 24 24"
              fill="none"
              stroke="#34D399"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <polyline points="4 17 10 11 4 5" />
              <line x1="12" y1="19" x2="20" y2="19" />
            </svg>
            <div style={{ fontSize: 13, fontWeight: 600, color: '#34D399' }}>Saved Queries</div>
          </div>
          <input
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            placeholder="Filter queries…"
            style={{
              width: '100%',
              background: 'rgba(52,211,153,0.05)',
              border: '1px solid rgba(52,211,153,0.15)',
              borderRadius: 8,
              padding: '6px 10px',
              color: '#fff',
              fontSize: 12,
              outline: 'none',
              boxSizing: 'border-box'
            }}
          />
        </div>
        <div style={{ flex: 1, overflowY: 'auto', padding: 8 }}>
          {filtered.map((q) => {
            const isActive = activeQuery?.id === q.id;
            const color = CATEGORY_COLORS[q.category];
            return (
              <div
                key={q.id}
                onClick={() => {
                  setActiveQuery(q);
                  setHasResult(false);
                }}
                style={{
                  padding: '10px 12px',
                  borderRadius: 10,
                  cursor: 'pointer',
                  background: isActive ? 'rgba(52,211,153,0.1)' : 'transparent',
                  borderLeft: isActive ? '2px solid #34D399' : '2px solid transparent',
                  marginBottom: 2,
                  transition: 'background 0.15s'
                }}
                onMouseEnter={(e) => {
                  if (!isActive)
                    (e.currentTarget as HTMLDivElement).style.background = 'rgba(52,211,153,0.05)';
                }}
                onMouseLeave={(e) => {
                  if (!isActive)
                    (e.currentTarget as HTMLDivElement).style.background = 'transparent';
                }}
              >
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 3 }}>
                  <span
                    style={{
                      fontSize: 10,
                      fontWeight: 600,
                      color,
                      background: `${color}18`,
                      borderRadius: 4,
                      padding: '1px 6px'
                    }}
                  >
                    {q.category}
                  </span>
                  {/* Star button */}
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      toggleStar({
                        id: `query-${q.id}`,
                        type: 'query',
                        title: q.name,
                        description: q.description
                      });
                    }}
                    title={isStarred(`query-${q.id}`) ? 'Remove from Starred' : 'Add to Starred'}
                    style={{
                      marginLeft: 'auto',
                      background: 'none',
                      border: 'none',
                      cursor: 'pointer',
                      padding: 2,
                      color: isStarred(`query-${q.id}`) ? '#FBBF24' : 'rgba(255,255,255,0.2)',
                      lineHeight: 1,
                      flexShrink: 0
                    }}
                  >
                    <svg
                      width="11"
                      height="11"
                      viewBox="0 0 24 24"
                      fill={isStarred(`query-${q.id}`) ? 'currentColor' : 'none'}
                      stroke="currentColor"
                      strokeWidth="2"
                    >
                      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                    </svg>
                  </button>
                </div>
                <div
                  style={{
                    fontSize: 12,
                    fontWeight: 600,
                    color: isActive ? '#fff' : 'rgba(255,255,255,0.75)',
                    lineHeight: 1.3
                  }}
                >
                  {q.name}
                </div>
                <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.3)', marginTop: 2 }}>
                  {q.runCount} runs · {q.lastRun}
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Query editor + results */}
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: 12, minWidth: 0 }}>
        {activeQuery ? (
          <>
            {/* Editor panel */}
            <div
              style={{
                background: 'rgba(4,16,12,.98)',
                border: '1px solid rgba(52,211,153,0.18)',
                borderRadius: 16,
                overflow: 'hidden'
              }}
            >
              <div
                style={{
                  padding: '12px 16px',
                  borderBottom: '1px solid rgba(52,211,153,0.12)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between'
                }}
              >
                <div>
                  <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>
                    {activeQuery.name}
                  </div>
                  <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 1 }}>
                    {activeQuery.description}
                  </div>
                </div>
                <button
                  onClick={handleRun}
                  disabled={isRunning}
                  style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 6,
                    background: isRunning ? 'rgba(52,211,153,0.2)' : '#34D399',
                    border: 'none',
                    borderRadius: 8,
                    color: isRunning ? '#34D399' : '#0A1A12',
                    fontSize: 12,
                    fontWeight: 700,
                    padding: '7px 16px',
                    cursor: isRunning ? 'wait' : 'pointer',
                    transition: 'all 0.15s'
                  }}
                >
                  {isRunning ? (
                    <>
                      <svg
                        width="12"
                        height="12"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2.5"
                        style={{ animation: 'spin 1s linear infinite' }}
                      >
                        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
                      </svg>
                      Running…
                    </>
                  ) : (
                    <>
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                        <polygon points="5,3 19,12 5,21" />
                      </svg>
                      Run Query
                    </>
                  )}
                </button>
              </div>
              {/* Mock SQL editor — terminal aesthetic with green keywords */}
              <div
                style={{
                  background: 'rgba(0,0,0,0.5)',
                  padding: '16px 20px',
                  fontFamily: '"Fira Code", "Cascadia Code", monospace',
                  fontSize: 13,
                  color: 'rgba(255,255,255,0.65)',
                  lineHeight: 1.8,
                  minHeight: 110,
                  borderBottom: '1px solid rgba(52,211,153,0.08)'
                }}
              >
                <span style={{ color: '#34D399', fontWeight: 700 }}>SELECT</span>
                {' broker_id, date, SUM(volume_usd) '}
                <span style={{ color: '#6EE7B7' }}>AS</span>
                {' volume, COUNT(*) '}
                <span style={{ color: '#6EE7B7' }}>AS</span>
                {' trades\n'}
                <span style={{ color: '#34D399', fontWeight: 700 }}>FROM</span>
                {' orderly.trades\n'}
                <span style={{ color: '#34D399', fontWeight: 700 }}>WHERE</span>
                {' date = '}
                <span style={{ color: '#FCD34D' }}>CURRENT_DATE</span>
                {'\n'}
                <span style={{ color: '#34D399', fontWeight: 700 }}>GROUP BY</span>
                {' broker_id, date\n'}
                <span style={{ color: '#34D399', fontWeight: 700 }}>ORDER BY</span>
                {' volume '}
                <span style={{ color: '#34D399', fontWeight: 700 }}>DESC</span>
              </div>
            </div>

            {/* Results panel */}
            {(hasResult || isRunning) && (
              <div
                style={{
                  background: 'rgba(4,16,12,.98)',
                  border: '1px solid rgba(52,211,153,0.18)',
                  borderRadius: 16,
                  overflow: 'hidden'
                }}
              >
                <div
                  style={{
                    padding: '12px 16px',
                    borderBottom: '1px solid rgba(52,211,153,0.12)',
                    display: 'flex',
                    alignItems: 'center',
                    gap: 8
                  }}
                >
                  <div style={{ fontSize: 13, fontWeight: 600, color: '#fff' }}>Results</div>
                  {hasResult && (
                    <span
                      style={{
                        fontSize: 11,
                        color: '#34d399',
                        background: 'rgba(52,211,153,0.1)',
                        borderRadius: 5,
                        padding: '1px 7px'
                      }}
                    >
                      5 rows
                    </span>
                  )}
                </div>
                <div style={{ padding: 16 }}>
                  {isRunning ? (
                    <div
                      style={{
                        textAlign: 'center',
                        color: 'rgba(255,255,255,0.4)',
                        fontSize: 13,
                        padding: '20px 0'
                      }}
                    >
                      Executing query…
                    </div>
                  ) : (
                    <pre
                      style={{
                        fontFamily: 'monospace',
                        fontSize: 12,
                        color: 'rgba(255,255,255,0.7)',
                        margin: 0,
                        overflowX: 'auto',
                        lineHeight: 1.8
                      }}
                    >
                      {MOCK_RESULT}
                    </pre>
                  )}
                </div>
              </div>
            )}
          </>
        ) : (
          <div
            style={{
              flex: 1,
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              background: 'rgba(4,16,12,.98)',
              border: '1px solid rgba(52,211,153,0.15)',
              borderRadius: 16,
              gap: 12,
              color: 'rgba(255,255,255,0.3)',
              fontSize: 14
            }}
          >
            <svg
              width="40"
              height="40"
              viewBox="0 0 24 24"
              fill="none"
              stroke="rgba(52,211,153,0.3)"
              strokeWidth="1.5"
            >
              <polyline points="4 17 10 11 4 5" />
              <line x1="12" y1="19" x2="20" y2="19" />
            </svg>
            Select a query from the list to view or run it
          </div>
        )}
      </div>
    </div>
  );
};
