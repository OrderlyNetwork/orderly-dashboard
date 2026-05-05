import { ColumnDef } from '@tanstack/react-table';
import { FC } from 'react';

import { SortableTable } from '../widgets/SortableTable';

type Activity = {
  txHash: string;
  type: 'Trade' | 'Liquidation' | 'Deposit' | 'Withdrawal';
  pair: string;
  size: string;
  price: string;
  broker: string;
  time: string;
};

const MOCK_ACTIVITY: Activity[] = [
  {
    txHash: '0xabc1…ef23',
    type: 'Trade',
    pair: 'BTC-PERP',
    size: '$124,500',
    price: '$67,234',
    broker: 'WOOFi',
    time: '2s ago'
  },
  {
    txHash: '0xdef4…ab56',
    type: 'Trade',
    pair: 'ETH-PERP',
    size: '$32,800',
    price: '$3,521',
    broker: 'Orderly DEX',
    time: '5s ago'
  },
  {
    txHash: '0x111a…bc78',
    type: 'Deposit',
    pair: '—',
    size: '$50,000',
    price: '—',
    broker: 'Aeterna',
    time: '12s ago'
  },
  {
    txHash: '0x222b…cd89',
    type: 'Trade',
    pair: 'SOL-PERP',
    size: '$18,200',
    price: '$178',
    broker: 'WOOFi',
    time: '18s ago'
  },
  {
    txHash: '0x333c…de90',
    type: 'Liquidation',
    pair: 'ARB-PERP',
    size: '$7,350',
    price: '$1.24',
    broker: 'FlowX',
    time: '23s ago'
  },
  {
    txHash: '0x444d…ef01',
    type: 'Trade',
    pair: 'BTC-PERP',
    size: '$89,000',
    price: '$67,190',
    broker: 'WOOFi',
    time: '31s ago'
  },
  {
    txHash: '0x555e…f012',
    type: 'Withdrawal',
    pair: '—',
    size: '$25,000',
    price: '—',
    broker: 'Orderly DEX',
    time: '45s ago'
  },
  {
    txHash: '0x666f…0123',
    type: 'Trade',
    pair: 'ETH-PERP',
    size: '$11,400',
    price: '$3,518',
    broker: 'Aeterna',
    time: '1m ago'
  },
  {
    txHash: '0x777a…1234',
    type: 'Trade',
    pair: 'OP-PERP',
    size: '$5,600',
    price: '$2.34',
    broker: 'FlowX',
    time: '1m ago'
  },
  {
    txHash: '0x888b…2345',
    type: 'Liquidation',
    pair: 'SOL-PERP',
    size: '$3,200',
    price: '$176',
    broker: 'WOOFi',
    time: '2m ago'
  }
];

const TYPE_COLORS: Record<Activity['type'], string> = {
  Trade: '#9C75FF',
  Liquidation: '#f87171',
  Deposit: '#34d399',
  Withdrawal: '#f59e0b'
};

const cardBgStyle = {
  background: 'rgba(20,15,35,.9)',
  border: '1px solid rgba(156,117,255,0.15)'
};

const columns: ColumnDef<Activity, unknown>[] = [
  {
    accessorKey: 'txHash',
    header: 'Tx Hash',
    cell: (info) => (
      <span className="font-mono text-xs text-[#9C75FF]">{info.getValue() as string}</span>
    ),
    enableSorting: false
  },
  {
    accessorKey: 'type',
    header: 'Type',
    cell: (info) => {
      const t = info.getValue() as Activity['type'];
      return (
        <span
          className="text-[11px] font-semibold rounded-md px-2 py-[2px]"
          style={{
            color: TYPE_COLORS[t],
            background: `${TYPE_COLORS[t]}18`
          }}
        >
          {t}
        </span>
      );
    }
  },
  { accessorKey: 'pair', header: 'Pair' },
  { accessorKey: 'size', header: 'Size' },
  { accessorKey: 'price', header: 'Price' },
  { accessorKey: 'broker', header: 'Broker' },
  {
    accessorKey: 'time',
    header: 'Time',
    cell: (info) => <span className="text-white/40 text-xs">{info.getValue() as string}</span>,
    enableSorting: false
  }
];

export const DiscoverView: FC = () => {
  return (
    <div className="flex flex-col gap-5">
      <div className="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-3">
        {[
          { label: 'Trades (24h)', value: '89,234' },
          { label: 'Liquidations (24h)', value: '342', color: '#f87171' },
          { label: 'Deposits (24h)', value: '$48.2M', color: '#34d399' },
          { label: 'Withdrawals (24h)', value: '$31.7M', color: '#f59e0b' }
        ].map((s) => (
          <div key={s.label} className="rounded-xl py-[14px] px-4" style={cardBgStyle}>
            <div className="text-[11px] text-white/40 uppercase tracking-[0.07em] font-semibold">
              {s.label}
            </div>
            <div className="text-xl font-bold mt-1" style={{ color: s.color ?? '#fff' }}>
              {s.value}
            </div>
          </div>
        ))}
      </div>

      <div className="rounded-2xl overflow-hidden" style={cardBgStyle}>
        <div className="px-5 py-4 border-b border-[rgba(156,117,255,0.1)] flex items-center gap-[10px]">
          <div className="w-2 h-2 rounded-full bg-[#34d399] shadow-[0_0_6px_#34d399]" />
          <span className="text-sm font-semibold text-white">Live Activity Feed</span>
          <span className="text-xs text-white/35">Mock data — updates in real time</span>
        </div>
        <SortableTable data={MOCK_ACTIVITY} columns={columns} rowKey={(r) => r.txHash} />
      </div>
    </div>
  );
};
