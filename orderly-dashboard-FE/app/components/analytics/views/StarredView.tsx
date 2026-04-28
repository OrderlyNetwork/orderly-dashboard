import { FC } from 'react';

import { type StarredItem } from '~/hooks/useStarred';

type StarredViewProps = {
  starred: StarredItem[];
  onRemove: (id: string) => void;
  onNavigate: (type: 'dashboard' | 'query') => void;
};

const TYPE_META = {
  dashboard: {
    color: '#9C75FF',
    bg: 'rgba(156,117,255,0.12)',
    border: 'rgba(156,117,255,0.25)',
    label: 'Dashboard',
    icon: (
      <svg
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <rect x="3" y="3" width="7" height="7" rx="1" />
        <rect x="14" y="3" width="7" height="7" rx="1" />
        <rect x="3" y="14" width="7" height="7" rx="1" />
        <rect x="14" y="14" width="7" height="7" rx="1" />
      </svg>
    )
  },
  query: {
    color: '#34D399',
    bg: 'rgba(52,211,153,0.12)',
    border: 'rgba(52,211,153,0.25)',
    label: 'Query',
    icon: (
      <svg
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <polyline points="4 17 10 11 4 5" />
        <line x1="12" y1="19" x2="20" y2="19" />
      </svg>
    )
  }
};

function timeAgo(ms: number) {
  const diff = Date.now() - ms;
  const m = Math.floor(diff / 60000);
  if (m < 1) return 'just now';
  if (m < 60) return `${m}m ago`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h ago`;
  return `${Math.floor(h / 24)}d ago`;
}

export const StarredView: FC<StarredViewProps> = ({ starred, onRemove, onNavigate }) => {
  const dashboards = starred.filter((s) => s.type === 'dashboard');
  const queries = starred.filter((s) => s.type === 'query');

  return (
    <div style={{ maxWidth: 860 }}>
      {/* Header */}
      <div style={{ marginBottom: 28 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 6 }}>
          <div
            style={{
              width: 36,
              height: 36,
              borderRadius: 10,
              background: 'rgba(251,191,36,0.15)',
              border: '1px solid rgba(251,191,36,0.3)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: '#FBBF24'
            }}
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="currentColor"
              stroke="currentColor"
              strokeWidth="1.5"
            >
              <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
            </svg>
          </div>
          <div>
            <h1 style={{ margin: 0, fontSize: 22, fontWeight: 700, color: '#fff' }}>Starred</h1>
            <p style={{ margin: 0, fontSize: 13, color: 'rgba(255,255,255,0.45)' }}>
              {starred.length === 0
                ? 'Your bookmarks — star any dashboard or query to save it here'
                : `${starred.length} saved item${starred.length !== 1 ? 's' : ''} — no login required`}
            </p>
          </div>
        </div>
      </div>

      {starred.length === 0 ? (
        /* Empty state */
        <div
          style={{
            border: '1px dashed rgba(255,255,255,0.1)',
            borderRadius: 16,
            padding: '60px 40px',
            textAlign: 'center'
          }}
        >
          <div style={{ fontSize: 40, marginBottom: 16, opacity: 0.3 }}>☆</div>
          <div
            style={{
              fontSize: 15,
              fontWeight: 600,
              color: 'rgba(255,255,255,0.5)',
              marginBottom: 8
            }}
          >
            Nothing starred yet
          </div>
          <div
            style={{
              fontSize: 13,
              color: 'rgba(255,255,255,0.28)',
              maxWidth: 320,
              margin: '0 auto',
              lineHeight: 1.6
            }}
          >
            Click the ☆ icon on any dashboard or query to bookmark it here. Saved locally — no login
            needed.
          </div>
          <div style={{ display: 'flex', gap: 10, justifyContent: 'center', marginTop: 24 }}>
            <button
              onClick={() => onNavigate('dashboard')}
              style={{
                padding: '8px 20px',
                borderRadius: 8,
                border: '1px solid rgba(156,117,255,0.3)',
                background: 'rgba(156,117,255,0.1)',
                color: '#9C75FF',
                fontSize: 13,
                fontWeight: 600,
                cursor: 'pointer'
              }}
            >
              Browse Dashboards
            </button>
            <button
              onClick={() => onNavigate('query')}
              style={{
                padding: '8px 20px',
                borderRadius: 8,
                border: '1px solid rgba(52,211,153,0.3)',
                background: 'rgba(52,211,153,0.1)',
                color: '#34D399',
                fontSize: 13,
                fontWeight: 600,
                cursor: 'pointer'
              }}
            >
              Browse Queries
            </button>
          </div>
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 32 }}>
          {/* Dashboard section */}
          {dashboards.length > 0 && (
            <section>
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: 8,
                  marginBottom: 12
                }}
              >
                <span style={{ color: '#9C75FF' }}>{TYPE_META.dashboard.icon}</span>
                <span
                  style={{
                    fontSize: 11,
                    fontWeight: 600,
                    color: 'rgba(255,255,255,0.3)',
                    letterSpacing: '0.08em',
                    textTransform: 'uppercase'
                  }}
                >
                  Dashboards · {dashboards.length}
                </span>
              </div>
              <div
                style={{
                  display: 'grid',
                  gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
                  gap: 12
                }}
              >
                {dashboards.map((item) => (
                  <StarredCard
                    key={item.id}
                    item={item}
                    onRemove={onRemove}
                    onNavigate={onNavigate}
                  />
                ))}
              </div>
            </section>
          )}

          {/* Queries section */}
          {queries.length > 0 && (
            <section>
              <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 12 }}>
                <span style={{ color: '#34D399' }}>{TYPE_META.query.icon}</span>
                <span
                  style={{
                    fontSize: 11,
                    fontWeight: 600,
                    color: 'rgba(255,255,255,0.3)',
                    letterSpacing: '0.08em',
                    textTransform: 'uppercase'
                  }}
                >
                  Queries · {queries.length}
                </span>
              </div>
              <div
                style={{
                  display: 'grid',
                  gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))',
                  gap: 12
                }}
              >
                {queries.map((item) => (
                  <StarredCard
                    key={item.id}
                    item={item}
                    onRemove={onRemove}
                    onNavigate={onNavigate}
                  />
                ))}
              </div>
            </section>
          )}
        </div>
      )}

      {/* Footer note */}
      {starred.length > 0 && (
        <p
          style={{
            marginTop: 32,
            fontSize: 12,
            color: 'rgba(255,255,255,0.2)',
            textAlign: 'center'
          }}
        >
          Starred items are saved in your browser's localStorage — no account required.
        </p>
      )}
    </div>
  );
};

const StarredCard: FC<{
  item: StarredItem;
  onRemove: (id: string) => void;
  onNavigate: (type: 'dashboard' | 'query') => void;
}> = ({ item, onRemove, onNavigate }) => {
  const meta = TYPE_META[item.type];

  return (
    <div
      onClick={() => onNavigate(item.type)}
      style={{
        padding: '14px 16px',
        borderRadius: 12,
        border: `1px solid ${meta.border}`,
        background: meta.bg,
        display: 'flex',
        flexDirection: 'column',
        gap: 8,
        position: 'relative',
        cursor: 'pointer',
        transition: 'border-color 0.15s, background 0.15s'
      }}
      onMouseEnter={(e) => {
        (e.currentTarget as HTMLDivElement).style.borderColor = meta.color + '60';
        (e.currentTarget as HTMLDivElement).style.background = meta.bg.replace('0.12', '0.18');
      }}
      onMouseLeave={(e) => {
        (e.currentTarget as HTMLDivElement).style.borderColor = meta.border;
        (e.currentTarget as HTMLDivElement).style.background = meta.bg;
      }}
    >
      {/* Type tag */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <span
          style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: 5,
            fontSize: 11,
            fontWeight: 600,
            color: meta.color,
            padding: '2px 8px',
            borderRadius: 4,
            background: 'rgba(0,0,0,0.2)'
          }}
        >
          {meta.icon}
          {meta.label}
        </span>
        {/* Remove star */}
        <button
          onClick={(e) => {
            e.stopPropagation();
            onRemove(item.id);
          }}
          title="Remove from starred"
          style={{
            background: 'none',
            border: 'none',
            cursor: 'pointer',
            padding: 4,
            color: '#FBBF24',
            opacity: 0.7,
            lineHeight: 1
          }}
        >
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="currentColor"
            stroke="currentColor"
            strokeWidth="1.5"
          >
            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
          </svg>
        </button>
      </div>

      <div style={{ fontSize: 14, fontWeight: 600, color: '#fff', lineHeight: 1.3 }}>
        {item.title}
      </div>
      <div
        style={{
          fontSize: 12,
          color: 'rgba(255,255,255,0.45)',
          lineHeight: 1.5,
          display: '-webkit-box',
          WebkitLineClamp: 2,
          WebkitBoxOrient: 'vertical',
          overflow: 'hidden'
        }}
      >
        {item.description}
      </div>
      <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.25)', marginTop: 2 }}>
        Starred {timeAgo(item.starredAt)}
      </div>
    </div>
  );
};
