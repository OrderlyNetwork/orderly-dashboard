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
        <span className="text-sm font-bold" style={{ color }}>
          {r <= 3 ? ['🥇', '🥈', '🥉'][r - 1] : `#${r}`}
        </span>
      );
    }
  },
  {
    accessorKey: 'broker',
    header: 'Broker',
    cell: (info) => (
      <div className="flex items-center gap-2.5">
        <div className="w-[30px] h-[30px] rounded-lg bg-[rgba(156,117,255,0.15)] border border-[rgba(156,117,255,0.2)] flex items-center justify-center text-[11px] font-bold text-[#9C75FF] shrink-0">
          {(info.getValue() as string).slice(0, 2).toUpperCase()}
        </div>
        <span className="font-semibold text-white">{info.getValue() as string}</span>
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
        <div className="flex items-center gap-2">
          <div className="flex-1 h-1 rounded-sm bg-[rgba(156,117,255,0.1)] max-w-[80px]">
            <div
              className="h-full rounded-sm bg-gradient-to-r from-[#9C75FF] to-[#6b3fcb]"
              style={{ width: `${pct}%` }}
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
        <span className="font-semibold text-[13px]" style={{ color: pos ? '#34d399' : '#f87171' }}>
          {pos ? '+' : ''}
          {v.toFixed(1)}%
        </span>
      );
    }
  }
];

export const BrokerLeaderboardView: FC = () => {
  return (
    <div className="flex flex-col gap-5">
      <div className="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-3">
        {[
          { label: 'Total Brokers', value: '24' },
          { label: 'Top Volume', value: '$125M', sub: 'WOOFi (24h)' },
          { label: 'Combined Volume', value: '$425.3M', sub: 'All brokers (24h)' },
          { label: 'New This Month', value: '3', sub: 'Active brokers' }
        ].map((s) => (
          <div
            key={s.label}
            className="bg-[rgba(20,15,35,.9)] border border-[rgba(156,117,255,0.15)] rounded-xl p-[14px_16px]"
          >
            <div className="text-[11px] text-[rgba(255,255,255,0.4)] uppercase tracking-[0.07em] font-semibold">
              {s.label}
            </div>
            <div className="text-xl font-bold text-white mt-1">{s.value}</div>
            {s.sub && (
              <div className="text-[11px] text-[rgba(255,255,255,0.35)] mt-0.5">{s.sub}</div>
            )}
          </div>
        ))}
      </div>

      <div className="bg-[rgba(20,15,35,.9)] border border-[rgba(156,117,255,0.15)] rounded-2xl overflow-hidden">
        <div className="p-[16px_20px] border-b border-[rgba(156,117,255,0.1)]">
          <div className="text-sm font-semibold text-white">Broker Rankings</div>
          <div className="text-xs text-[rgba(255,255,255,0.35)] mt-0.5">
            Click column headers to sort · 24h snapshot
          </div>
        </div>
        <SortableTable data={BROKER_DATA} columns={columns} rowKey={(r) => r.brokerId} />
      </div>
    </div>
  );
};
