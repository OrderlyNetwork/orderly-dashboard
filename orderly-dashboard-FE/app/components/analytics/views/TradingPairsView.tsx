import { ColumnDef } from '@tanstack/react-table';
import { FC } from 'react';

import { SortableTable } from '../widgets/SortableTable';

export type PairRow = {
  pair: string;
  price: number;
  volume24h: number;
  change24h: number;
  openInterest: number;
  fundingRate: number;
  trades24h: number;
};

export const PAIRS_DATA: PairRow[] = [
  { pair: 'BTC-PERP', price: 67_234.5, volume24h: 145_000_000, change24h: 2.34, openInterest: 450_000_000, fundingRate: 0.0012, trades24h: 28_430 },
  { pair: 'ETH-PERP', price: 3_521.2, volume24h: 98_000_000, change24h: -0.87, openInterest: 310_000_000, fundingRate: 0.0008, trades24h: 22_100 },
  { pair: 'SOL-PERP', price: 178.4, volume24h: 52_000_000, change24h: 5.62, openInterest: 120_000_000, fundingRate: 0.0021, trades24h: 15_600 },
  { pair: 'ARB-PERP', price: 1.24, volume24h: 31_000_000, change24h: -2.15, openInterest: 78_000_000, fundingRate: -0.0004, trades24h: 9_800 },
  { pair: 'OP-PERP', price: 2.34, volume24h: 24_000_000, change24h: 1.08, openInterest: 62_000_000, fundingRate: 0.0006, trades24h: 7_200 },
  { pair: 'AVAX-PERP', price: 38.7, volume24h: 18_500_000, change24h: 3.41, openInterest: 44_000_000, fundingRate: 0.0014, trades24h: 5_100 },
  { pair: 'MATIC-PERP', price: 0.87, volume24h: 14_200_000, change24h: -1.56, openInterest: 31_000_000, fundingRate: -0.0002, trades24h: 4_300 },
  { pair: 'SUI-PERP', price: 1.42, volume24h: 12_800_000, change24h: 8.94, openInterest: 28_000_000, fundingRate: 0.0033, trades24h: 6_700 },
  { pair: 'NEAR-PERP', price: 6.23, volume24h: 9_400_000, change24h: 0.54, openInterest: 19_000_000, fundingRate: 0.0005, trades24h: 2_900 },
  { pair: 'TIA-PERP', price: 11.8, volume24h: 8_100_000, change24h: -4.22, openInterest: 17_500_000, fundingRate: -0.0009, trades24h: 2_400 },
  { pair: 'INJ-PERP', price: 24.5, volume24h: 7_600_000, change24h: 2.88, openInterest: 15_000_000, fundingRate: 0.0011, trades24h: 1_900 },
  { pair: 'DOGE-PERP', price: 0.162, volume24h: 6_200_000, change24h: -0.62, openInterest: 12_000_000, fundingRate: 0.0001, trades24h: 3_200 }
];

function fmtUsd(n: number) {
  if (n >= 1e9) return `$${(n / 1e9).toFixed(2)}B`;
  if (n >= 1e6) return `$${(n / 1e6).toFixed(1)}M`;
  return `$${n.toLocaleString()}`;
}

function fmtPrice(n: number) {
  if (n >= 1000) return `$${n.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
  if (n >= 1) return `$${n.toFixed(2)}`;
  return `$${n.toFixed(4)}`;
}

const columns: ColumnDef<PairRow, unknown>[] = [
  {
    accessorKey: 'pair',
    header: 'Pair',
    cell: (info) => (
      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
        <div
          style={{
            width: 28,
            height: 28,
            borderRadius: 7,
            background: 'linear-gradient(135deg, rgba(156,117,255,0.25), rgba(156,117,255,0.08))',
            border: '1px solid rgba(156,117,255,0.2)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            fontSize: 9,
            fontWeight: 800,
            color: '#9C75FF',
            letterSpacing: '-0.03em',
            flexShrink: 0
          }}
        >
          {(info.getValue() as string).replace('-PERP', '').slice(0, 3)}
        </div>
        <span style={{ fontWeight: 600, color: '#fff' }}>{info.getValue() as string}</span>
      </div>
    )
  },
  {
    accessorKey: 'price',
    header: 'Price',
    cell: (info) => fmtPrice(info.getValue() as number)
  },
  {
    accessorKey: 'change24h',
    header: '24h Change',
    cell: (info) => {
      const v = info.getValue() as number;
      const pos = v >= 0;
      return (
        <span style={{ color: pos ? '#34d399' : '#f87171', fontWeight: 600 }}>
          {pos ? '+' : ''}{v.toFixed(2)}%
        </span>
      );
    }
  },
  {
    accessorKey: 'volume24h',
    header: 'Volume (24h)',
    cell: (info) => fmtUsd(info.getValue() as number)
  },
  {
    accessorKey: 'trades24h',
    header: 'Trades (24h)',
    cell: (info) => (info.getValue() as number).toLocaleString()
  },
  {
    accessorKey: 'openInterest',
    header: 'Open Interest',
    cell: (info) => fmtUsd(info.getValue() as number)
  },
  {
    accessorKey: 'fundingRate',
    header: 'Funding Rate',
    cell: (info) => {
      const v = info.getValue() as number;
      const pos = v >= 0;
      return (
        <span style={{ color: pos ? '#34d399' : '#f87171', fontFamily: 'monospace', fontSize: 12 }}>
          {pos ? '+' : ''}{(v * 100).toFixed(4)}%
        </span>
      );
    }
  }
];

export const TradingPairsView: FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: 20 }}>
      {/* Header stats */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))',
          gap: 12
        }}
      >
        {[
          { label: 'Active Pairs', value: `${PAIRS_DATA.length}` },
          { label: 'Total Volume (24h)', value: '$425.3M' },
          { label: 'Total OI', value: '$2.14B' },
          { label: 'Best Performer', value: 'SUI-PERP', sub: '+8.94%' }
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
            <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)', textTransform: 'uppercase', letterSpacing: '0.07em', fontWeight: 600 }}>
              {s.label}
            </div>
            <div style={{ fontSize: 20, fontWeight: 700, color: '#fff', marginTop: 4 }}>{s.value}</div>
            {s.sub && (
              <div style={{ fontSize: 11, color: '#34d399', marginTop: 2 }}>{s.sub}</div>
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
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>All Perpetual Pairs</div>
          <div style={{ fontSize: 12, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
            Click column headers to sort · 24h snapshot · Funding rate per 8h
          </div>
        </div>
        <SortableTable data={PAIRS_DATA} columns={columns} rowKey={(r) => r.pair} />
      </div>
    </div>
  );
};
