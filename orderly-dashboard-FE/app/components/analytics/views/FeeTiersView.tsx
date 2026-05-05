import { ColumnDef } from '@tanstack/react-table';
import { FC } from 'react';

import { SortableTable } from '../widgets/SortableTable';

export type FeeTierRow = {
  tier: string;
  makerFee: string;
  takerFee: string;
  volumeReq: string;
  volumeReqRaw: number;
  users: number;
  rebate: string;
};

export const FEE_TIERS_DATA: FeeTierRow[] = [
  {
    tier: 'Basic',
    makerFee: '0.02%',
    takerFee: '0.06%',
    volumeReq: '$0',
    volumeReqRaw: 0,
    users: 45_200,
    rebate: '0%'
  },
  {
    tier: 'Bronze',
    makerFee: '0.018%',
    takerFee: '0.055%',
    volumeReq: '$500K',
    volumeReqRaw: 500_000,
    users: 8_400,
    rebate: '5%'
  },
  {
    tier: 'Silver',
    makerFee: '0.015%',
    takerFee: '0.048%',
    volumeReq: '$2M',
    volumeReqRaw: 2_000_000,
    users: 3_100,
    rebate: '10%'
  },
  {
    tier: 'Gold',
    makerFee: '0.012%',
    takerFee: '0.040%',
    volumeReq: '$10M',
    volumeReqRaw: 10_000_000,
    users: 980,
    rebate: '18%'
  },
  {
    tier: 'Platinum',
    makerFee: '0.008%',
    takerFee: '0.030%',
    volumeReq: '$50M',
    volumeReqRaw: 50_000_000,
    users: 210,
    rebate: '28%'
  },
  {
    tier: 'Diamond',
    makerFee: '0.005%',
    takerFee: '0.020%',
    volumeReq: '$100M',
    volumeReqRaw: 100_000_000,
    users: 47,
    rebate: '40%'
  },
  {
    tier: 'Market Maker',
    makerFee: '-0.002%',
    takerFee: '0.010%',
    volumeReq: 'By Application',
    volumeReqRaw: 999_999_999,
    users: 12,
    rebate: '50%'
  }
];

const TIER_COLORS: Record<string, string> = {
  Basic: '#94a3b8',
  Bronze: '#cd7c3a',
  Silver: '#e2e8f0',
  Gold: '#f59e0b',
  Platinum: '#7dd3fc',
  Diamond: '#a78bfa',
  'Market Maker': '#34d399'
};

const columns: ColumnDef<FeeTierRow, unknown>[] = [
  {
    accessorKey: 'tier',
    header: 'Tier',
    cell: (info) => {
      const t = info.getValue() as string;
      const color = TIER_COLORS[t] ?? '#9C75FF';
      return (
        <div className="flex items-center gap-2">
          <div
            className="w-2.5 h-2.5 rounded-full shrink-0"
            style={{
              background: color,
              boxShadow: `0 0 6px ${color}80`
            }}
          />
          <span className="font-bold" style={{ color }}>
            {t}
          </span>
        </div>
      );
    }
  },
  {
    accessorKey: 'makerFee',
    header: 'Maker Fee',
    cell: (info) => {
      const v = info.getValue() as string;
      const isNegative = v.startsWith('-');
      return (
        <span
          className="font-mono font-semibold"
          style={{ color: isNegative ? '#34d399' : 'rgba(255,255,255,0.85)' }}
        >
          {v}
        </span>
      );
    }
  },
  {
    accessorKey: 'takerFee',
    header: 'Taker Fee',
    cell: (info) => <span className="font-mono font-semibold">{info.getValue() as string}</span>
  },
  {
    accessorKey: 'volumeReqRaw',
    header: 'Volume Required (30d)',
    cell: (info) => {
      const row = info.row.original;
      return <span>{row.volumeReq}</span>;
    }
  },
  {
    accessorKey: 'rebate',
    header: 'Fee Rebate',
    cell: (info) => {
      const v = info.getValue() as string;
      const pct = parseFloat(v);
      return (
        <span
          className="font-semibold"
          style={{ color: pct > 0 ? '#34d399' : 'rgba(255,255,255,0.4)' }}
        >
          {v}
        </span>
      );
    }
  },
  {
    accessorKey: 'users',
    header: 'Users',
    cell: (info) => (info.getValue() as number).toLocaleString()
  }
];

export const FeeTiersView: FC = () => {
  const totalUsers = FEE_TIERS_DATA.reduce((s, r) => s + r.users, 0);

  return (
    <div className="flex flex-col gap-5">
      <div
        className="rounded-2xl p-6 border border-[rgba(156,117,255,0.15)]"
        style={{ background: 'rgba(20,15,35,.9)' }}
      >
        <div className="text-sm font-semibold text-white mb-4">User Distribution by Tier</div>
        <div className="flex flex-col gap-2.5">
          {FEE_TIERS_DATA.map((tier) => {
            const pct = (tier.users / totalUsers) * 100;
            const color = TIER_COLORS[tier.tier] ?? '#9C75FF';
            return (
              <div key={tier.tier} className="flex items-center gap-3">
                <div className="w-20 text-xs font-semibold shrink-0" style={{ color }}>
                  {tier.tier}
                </div>
                <div className="flex-1 h-2 rounded bg-white/[0.06] overflow-hidden">
                  <div
                    className="h-full rounded opacity-80 transition-[width] duration-[600ms]"
                    style={{ width: `${pct}%`, background: color }}
                  />
                </div>
                <div className="w-[60px] text-xs text-white/50 text-right shrink-0">
                  {tier.users.toLocaleString()}
                </div>
                <div className="w-10 text-[11px] text-white/35 text-right shrink-0">
                  {pct.toFixed(1)}%
                </div>
              </div>
            );
          })}
        </div>
      </div>

      <div className="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-3">
        {[
          { label: 'Total Users', value: totalUsers.toLocaleString() },
          { label: 'Fee Tiers', value: `${FEE_TIERS_DATA.length}` },
          { label: 'Lowest Maker Fee', value: '-0.002%', sub: 'Market Maker tier' },
          { label: 'Max Rebate', value: '50%', sub: 'Market Maker tier' }
        ].map((s) => (
          <div
            key={s.label}
            className="rounded-xl px-4 py-3.5 border border-[rgba(156,117,255,0.15)]"
            style={{ background: 'rgba(20,15,35,.9)' }}
          >
            <div className="text-[11px] text-white/40 uppercase tracking-[0.07em] font-semibold">
              {s.label}
            </div>
            <div className="text-xl font-bold text-white mt-1">{s.value}</div>
            {s.sub && <div className="text-[11px] text-white/35 mt-0.5">{s.sub}</div>}
          </div>
        ))}
      </div>

      <div
        className="rounded-2xl overflow-hidden border border-[rgba(156,117,255,0.15)]"
        style={{ background: 'rgba(20,15,35,.9)' }}
      >
        <div className="px-5 py-4 border-b border-[rgba(156,117,255,0.1)]">
          <div className="text-sm font-semibold text-white">Fee Schedule</div>
          <div className="text-xs text-white/35 mt-0.5">
            Click column headers to sort · Negative maker fee = rebate paid to maker
          </div>
        </div>
        <SortableTable data={FEE_TIERS_DATA} columns={columns} rowKey={(r) => r.tier} />
      </div>
    </div>
  );
};
