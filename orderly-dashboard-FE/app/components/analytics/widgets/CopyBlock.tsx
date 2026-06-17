import { useState } from 'react';

type CopyBlockProps = {
  widgetId: string;
};

export const CopyBlock = ({ widgetId }: CopyBlockProps) => {
  const [copied, setCopied] = useState(false);
  const [copyHovered, setCopyHovered] = useState(false);
  const origin =
    typeof window !== 'undefined' ? window.location.origin : 'https://dashboard.orderly.network';
  const iframeCode = `<iframe src="${origin}/widget/${widgetId}?embed=true" width="800" height="400" frameborder="0"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(iframeCode).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  return (
    <div>
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          marginBottom: 8
        }}
      >
        <span style={{ fontSize: 12, fontWeight: 600, color: 'rgba(255,255,255,0.5)' }}>
          Embed Code
        </span>
        <button
          onClick={handleCopy}
          onMouseEnter={() => setCopyHovered(true)}
          onMouseLeave={() => setCopyHovered(false)}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: 6,
            background: copied ? 'rgba(52,211,153,0.15)' : copyHovered ? 'rgba(255,255,255,0.1)' : 'transparent',
            border: `1px solid ${copied ? 'rgba(52,211,153,0.6)' : 'rgba(255,255,255,0.6)'}`,
            borderRadius: 999,
            color: copied ? '#00dea3' : '#fff',
            fontSize: 12,
            fontWeight: 600,
            padding: '6px 14px',
            cursor: 'pointer',
            transition: 'all 0.15s',
            transform: copyHovered && !copied ? 'translateY(-1px)' : 'none'
          }}
        >
          {copied ? (
            <>
              <svg
                width="12"
                height="12"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="3"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <polyline points="20 6 9 17 4 12" />
              </svg>
              Copied
            </>
          ) : (
            <>
              <svg
                width="12"
                height="12"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
              Copy
            </>
          )}
        </button>
      </div>
      <pre
        style={{
          background: 'rgba(20,15,35,0.9)',
          border: 'none',
          borderRadius: 10,
          padding: '14px 18px',
          fontSize: 13,
          fontFamily: '"SF Mono", "Fira Code", "Fira Mono", Menlo, monospace',
          color: 'rgba(255,255,255,0.8)',
          margin: 0,
          overflowX: 'auto',
          lineHeight: 1.6,
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-all'
        }}
      >
        {iframeCode}
      </pre>
    </div>
  );
};
