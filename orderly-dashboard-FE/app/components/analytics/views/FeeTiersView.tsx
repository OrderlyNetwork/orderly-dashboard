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
  { tier: 'Basic', makerFee: '0.02%', takerFee: '0.06%', volumeReq: '$0', volumeReqRaw: 0, users: 45_200, rebate: '0%' },
  { tier: 'Bronze', makerFee: '0.018%', takerFee: '0.055%', volumeReq: '$500K', volumeReqRaw: 500_000, users: 8_400, rebate: '5%' },
  { tier: 'Silver', makerFee: '0.015%', takerFee: '0.048%', volumeReq: '$2M', volumeReqRaw: 2_000_000, users: 3_100, rebate: '10%' },
  { tier: 'Gold', makerFee: '0.012%', takerFee: '0.040%', volumeReq: '$10M', volumeReqRaw: 10_000_000, users: 980, rebate: '18%' },
  { tier: 'Platinum', makerFee: '0.008%', takerFee: '0.030%', volumeReq: '$50M', volumeReqRaw: 50_000_000, users: 210, rebate: '28%' },
  { tier: 'Diamond', makerFee: '0.005%', takerFee: '0.020%', volumeReq: '$100M', volumeReqRaw: 100_000_000, users: 47, rebate: '40%' },
  { tier: 'Market Maker', makerFee: '-0.002%', takerFee: '0.010%', volumeReq: 'By Application', volumeReqRaw: 999_999_999, users: 12, rebate: '50%' }
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
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <div
            style={{
              width: 10,
              height: 10,
              borderRadius: '50%',
              background: color,
              boxShadow: `0 0 6px ${color}80`,
              flexShrink: 0
            }}
          />
          <span style={{ fontWeight: 700, color }}>{t}</span>
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
        <span style={{ fontFamily: 'monospace', color: isNegative ? '#34d399' : 'rgba(255,255,255,0.85)', fontWeight: 600 }}>
          {v}
        </span>
      );
    }
  },
  {
    accessorKey: 'takerFee',
    header: 'Taker Fee',
    cell: (info) => (
      <span style={{ fontFamily: 'monospace', fontWeight: 600 }}>{info.getValue() as string}</span>
    )
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
        <span style={{ color: pct > 0 ? '#34d399' : 'rgba(255,255,255,0.4)', fontWeight: 600 }}>
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
    <div style={{ display: 'flex', flexDirection: 'column', gap: 20 }}>
      {/* User distribution */}
      <div
        style={{
          background: 'rgba(20,15,35,.9)',
          border: '1px solid rgba(156,117,255,0.15)',
          borderRadius: 16,
          padding: 24
        }}
      >
        <div style={{ fontSize: 14, fontWeight: 600, color: '#fff', marginBottom: 16 }}>User Distribution by Tier</div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
          {FEE_TIERS_DATA.map((tier) => {
            const pct = (tier.users / totalUsers) * 100;
            const color = TIER_COLORS[tier.tier] ?? '#9C75FF';
            return (
              <div key={tier.tier} style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
                <div style={{ width: 80, fontSize: 12, color, fontWeight: 600, flexShrink: 0 }}>
                  {tier.tier}
                </div>
                <div style={{ flex: 1, height: 8, borderRadius: 4, background: 'rgba(255,255,255,0.06)', overflow: 'hidden' }}>
                  <div
                    style={{
                      height: '100%',
                      width: `${pct}%`,
                      borderRadius: 4,
                      background: color,
                      transition: 'width 0.6s ease',
                      opacity: 0.8
                    }}
                  />
                </div>
                <div style={{ width: 60, fontSize: 12, color: 'rgba(255,255,255,0.5)', textAlign: 'right', flexShrink: 0 }}>
                  {tier.users.toLocaleString()}
                </div>
                <div style={{ width: 40, fontSize: 11, color: 'rgba(255,255,255,0.35)', textAlign: 'right', flexShrink: 0 }}>
                  {pct.toFixed(1)}%
                </div>
              </div>
            );
          })}
        </div>
      </div>

      {/* Summary */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(160px, 1fr))', gap: 12 }}>
        {[
          { label: 'Total Users', value: totalUsers.toLocaleString() },
          { label: 'Fee Tiers', value: `${FEE_TIERS_DATA.length}` },
          { label: 'Lowest Maker Fee', value: '-0.002%', sub: 'Market Maker tier' },
          { label: 'Max Rebate', value: '50%', sub: 'Market Maker tier' }
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
            {s.sub && <div style={{ fontSize: 11, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>{s.sub}</div>}
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
          <div style={{ fontSize: 14, fontWeight: 600, color: '#fff' }}>Fee Schedule</div>
          <div style={{ fontSize: 12, color: 'rgba(255,255,255,0.35)', marginTop: 2 }}>
            Click column headers to sort · Negative maker fee = rebate paid to maker
          </div>
        </div>
        <SortableTable data={FEE_TIERS_DATA} columns={columns} rowKey={(r) => r.tier} />
      </div>
    </div>
  );
};
