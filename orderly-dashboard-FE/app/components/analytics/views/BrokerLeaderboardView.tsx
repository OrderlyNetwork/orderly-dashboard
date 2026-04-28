import { ColumnDef } from '@tanstack/react-table';
import { FC } from 'react';

import { SortableTable } from '../widgets/SortableTable';

export type BrokerRow = {
  rank: number;
  broker: string;
  brokerId: string;
  volume24h: number;
  traders: number;
  revenue: number;
  marketShare: number;
  change7d: number;
};

export const BROKER_DATA: BrokerRow[] = [
  {
    rank: 1,
    broker: 'WOOFi',
    brokerId: 'woofi',
    volume24h: 125_000_000,
    traders: 3421,
    revenue: 62_500,
    marketShare: 29.4,
    change7d: 4.2
  },
  {
    rank: 2,
    broker: 'Orderly DEX',
    brokerId: 'orderly',
    volume24h: 98_000_000,
    traders: 2156,
    revenue: 49_000,
    marketShare: 23.0,
    change7d: -1.1
  },
  {
    rank: 3,
    broker: 'Aeterna',
    brokerId: 'aeterna',
    volume24h: 72_000_000,
    traders: 1830,
    revenue: 36_000,
    marketShare: 16.9,
    change7d: 8.5
  },
  {
    rank: 4,
    broker: 'FlowX',
    brokerId: 'flowx',
    volume24h: 55_000_000,
    traders: 1244,
    revenue: 27_500,
    marketShare: 12.9,
    change7d: 2.3
  },
  {
    rank: 5,
    broker: 'Perp Pro',
    brokerId: 'perppro',
    volume24h: 38_000_000,
    traders: 987,
    revenue: 19_000,
    marketShare: 8.9,
    change7d: -3.6
  },
  {
    rank: 6,
    broker: 'TradeX',
    brokerId: 'tradex',
    volume24h: 21_000_000,
    traders: 654,
    revenue: 10_500,
    marketShare: 4.9,
    change7d: 1.8
  },
  {
    rank: 7,
    broker: 'DeltaDex',
    brokerId: 'deltadex',
    volume24h: 12_400_000,
    traders: 398,
    revenue: 6_200,
    marketShare: 2.9,
    change7d: 12.4
  },
  {
    rank: 8,
    broker: 'NovaTrade',
    brokerId: 'novatrade',
    volume24h: 3_900_000,
    traders: 157,
    revenue: 1_950,
    marketShare: 0.9,
    change7d: -0.5
  }
];

function fmtUsd(n: number) {
  if (n >= 1e6) return `$${(n / 1e6).toFixed(1)}M`;
  if (n >= 1e3) return `$${(n / 1e3).toFixed(1)}K`;
  return `$${n.toLocaleString()}`;
}

const columns: ColumnDef<BrokerRow, unknown>[] = [
  {
    accessorKey: 'rank',
    header: 'Rank',
    cell: (info) => {
      const r = info.getValue() as number;
      const colors = ['#f59e0b', '#94a3b8', '#cd7c3a'];
      const color = r <= 3 ? colors[r - 1] : 'rgba(255,255,255,0.3)';
      return (
        <span style={{ fontWeight: 700, color, fontSize: 14 }}>
          {r <= 3 ? ['🥇', '🥈', '🥉'][r - 1] : `#${r}`}
        </span>
      );
    }
  },
  {
    accessorKey: 'broker',
    header: 'Broker',
    cell: (info) => (
      <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
        <div
          style={{
            width: 30,
            height: 30,
            borderRadius: 8,
            background: 'rgba(156,117,255,0.15)',
            border: '1px solid rgba(156,117,255,0.2)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 11,
            fontWeight: 700,
            color: '#9C75FF',
            flexShrink: 0
          }}
        >
          {(info.getValue() as string).slice(0, 2).toUpperCase()}
        </div>
        <span style={{ fontWeight: 600, color: '#fff' }}>{info.getValue() as string}</span>
      </div>
    )
  },
  {
    accessorKey: 'volume24h',
    header: 'Volume (24h)',
    cell: (info) => fmtUsd(info.getValue() as number)
  },
  {
    accessorKey: 'traders',
    header: 'Traders',
    cell: (info) => (info.getValue() as number).toLocaleString()
  },
  {
    accessorKey: 'revenue',
    header: 'Revenue (24h)',
    cell: (info) => fmtUsd(info.getValue() as number)
  },
  {
    accessorKey: 'marketShare',
    header: 'Market Share',
    cell: (info) => {
      const pct = info.getValue() as number;
      return (
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div
            style={{
              flex: 1,
              height: 4,
              borderRadius: 2,
              background: 'rgba(156,117,255,0.1)',
              maxWidth: 80
            }}
          >
            <div
              style={{
                height: '100%',
                width: `${pct}%`,
                borderRadius: 2,
                background: 'linear-gradient(90deg, #9C75FF, #6b3fcb)'
              }}
            />
          </div>
          <span>{pct.toFixed(1)}%</span>
        </div>
      );
    }
  },
  {
    accessorKey: 'change7d',
    header: '7D Change',
    cell: (info) => {
      const v = info.getValue() as number;
      const pos = v >= 0;
      return (
        <span
          style={{
            color: pos ? '#34d399' : '#f87171',
            fontWeight: 600,
            fontSize: 13
          }}
        >
          {pos ? '+' : ''}
          {v.toFixed(1)}%
        </span>
      );
    }
  }
];

export const BrokerLeaderboardView: FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 20 }}>
      {/* Summary cards */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: 12
        }}
      >
        {[
          { label: 'Total Brokers', value: '24' },
          { label: 'Top Volume', value: '$125M', sub: 'WOOFi (24h)' },
          { label: 'Combined Volume', value: '$425.3M', sub: 'All brokers (24h)' },
          { label: 'New This Month', value: '3', sub: 'Active brokers' }
        ].map((s) => (
          <div
            key={s.label}
            style={{
              background: 'rgba(20,15,35,.9)',
              border: '1px solid rgba(156,117,255,0.15)',
              borderRadius: 12,
              padding: '14px 16px'
            }}
          >
            <div
              style={{
                fontSize: 11,
                color: 'rgba(255,255,255,0.4)',
                textTransform: 'uppercase',
                letterSpacing: '0.07em',
                fontWeight: 600
              }}
            >
              {s.label}
            </div>
            <div style={{ fontSize: 20, fontWeight: 700, color: '#fff', marginTop: 4 }}>
              {s.value}
            </div>
            {s.sub && (
              <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
                {s.sub}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Table */}
      <div
        style={{
          background: 'rgba(20,15,35,.9)',
          border: '1px solid rgba(156,117,255,0.15)',
          borderRadius: 16,
          overflow: 'hidden'
        }}
      >
        <div style={{ padding: '16px 20px', borderBottom: '1px solid rgba(156,117,255,0.1)' }}>
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>Broker Rankings</div>
          <div style={{ fontSize: 12, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
            Click column headers to sort · 24h snapshot
          </div>
        </div>
        <SortableTable data={BROKER_DATA} columns={columns} rowKey={(r) => r.brokerId} />
      </div>
    </div>
  );
};
