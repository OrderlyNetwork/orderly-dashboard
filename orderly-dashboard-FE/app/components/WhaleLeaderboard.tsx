import { Table, Tooltip } from '@radix-ui/themes';
import { Link } from '@remix-run/react';
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable
} from '@tanstack/react-table';
import { useMemo, useState, FC, useCallback, useEffect, useRef } from 'react';

import { useAppState } from '~/App';
import { Spinner } from '~/components';
import { WhaleDetailModal } from '~/components/WhaleDetailModal';
import { TopAddressEntry } from '~/hooks/usePublicInfo';
import { base64UrlSafeEncode } from '~/util';

const MaterialContentCopyIcon = ({
  className,
  onClick
}: {
  className?: string;
  onClick?: () => void;
}) => (
  <svg
    className={className}
    viewBox="0 0 24 24"
    fill="currentColor"
    width="1em"
    height="1em"
    onClick={onClick}
    style={{ cursor: onClick ? 'pointer' : undefined }}
  >
    <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z" />
  </svg>
);

export const WhaleLeaderboard: FC = () => {
  const { evmApiUrl } = useAppState();
  const [duration, setDuration] = useState<'24h' | '7d' | '30d'>('24h');
  const [sorting, setSorting] = useState<SortingState>([{ id: 'pnl', desc: true }]);
  const [selectedWhale, setSelectedWhale] = useState<{
    address: string;
    brokerId?: string;
  } | null>(null);
  const [allWhales, setAllWhales] = useState<TopAddressEntry[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [lastUpdated, setLastUpdated] = useState<number>(0);
  const hasFetched = useRef(false);

  const fetchAllPages = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    const allRows: TopAddressEntry[] = [];
    let cursor: string | undefined;
    let lastTs = 0;

    try {
      do {
        const params: Record<string, unknown> = {
          sort_by: 'notional',
          min_notional: 50000,
          limit: 200
        };
        if (cursor) params.cursor = cursor;

        const res = await fetch(`${evmApiUrl}/v1/public/query`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ type: 'topAddresses', ...params })
        });
        const json = await res.json();
        if (!json.success) throw new Error(json.message || 'Failed to fetch whale data');

        const data = json.data;
        allRows.push(...data.rows);
        lastTs = data.last_updated_time;
        cursor = data.next_cursor;
      } while (cursor);

      setAllWhales(allRows);
      setLastUpdated(lastTs);
    } catch (err) {
      setError(err as Error);
    } finally {
      setIsLoading(false);
    }
  }, [evmApiUrl]);

  useEffect(() => {
    if (!hasFetched.current) {
      hasFetched.current = true;
      fetchAllPages();
    }
  }, [fetchAllPages]);

  useEffect(() => {
    setSorting([{ id: 'pnl', desc: true }]);
  }, [duration]);

  const formatNumberShort = (value: string | number) => {
    const num = typeof value === 'string' ? parseFloat(value) : value;
    if (isNaN(num)) return '-';
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      notation: 'compact',
      maximumFractionDigits: 3,
      minimumFractionDigits: 0
    }).format(num);
  };

  const formatPercent = (value: number | null) => {
    if (value === null || value === undefined) return '-';
    return `${(value * 100).toFixed(1)}%`;
  };

  const formatAddress = (address: string | undefined) => {
    if (!address) return '-';
    return `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
  };

  const columns = useMemo<ColumnDef<TopAddressEntry>[]>(
    () => [
      {
        accessorKey: 'address',
        header: 'Address',
        cell: ({ row }) => (
          <div className="flex items-center gap-1">
            <Link
              to={(() => {
                const addr = row.original.address;
                const isSol = addr.match(/^[0-9a-zA-Z]{43,44}$/);
                if (isSol) {
                  return `/search?q=${base64UrlSafeEncode(addr)}&chain_namespace=sol`;
                } else {
                  return `/search?q=${addr}&chain_namespace=evm`;
                }
              })()}
              className="font-address text-sm"
            >
              {formatAddress(row.original.address)}
            </Link>
            <Tooltip content="Copy address">
              <MaterialContentCopyIcon
                className="w-3 h-3 text-gray-400 hover:text-white cursor-pointer"
                onClick={() => navigator.clipboard.writeText(row.original.address)}
              />
            </Tooltip>
            <Tooltip content="View whale details">
              <button
                className="w-4 h-4 flex items-center justify-center text-gray-400 hover:text-purple-400 cursor-pointer bg-transparent border-none p-0"
                onClick={() =>
                  setSelectedWhale({
                    address: row.original.address,
                    brokerId: row.original.broker_id
                  })
                }
              >
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
                  <path d="M2 12c0 0 4-8 10-8s10 8 10 8-4 8-10 8-10-8-10-8z" />
                  <circle cx="12" cy="12" r="3" />
                </svg>
              </button>
            </Tooltip>
          </div>
        ),
        enableSorting: false
      },
      {
        accessorKey: 'pnl',
        header: `${duration} PnL`,
        cell: ({ row }) => {
          const value = parseFloat(
            duration === '24h'
              ? row.original.pnl_24h
              : duration === '7d'
                ? row.original.pnl_7d
                : row.original.pnl_30d
          );
          return (
            <span style={{ color: value >= 0 ? '#00dea3' : '#FF6390' }}>
              {formatNumberShort(
                duration === '24h'
                  ? row.original.pnl_24h
                  : duration === '7d'
                    ? row.original.pnl_7d
                    : row.original.pnl_30d
              )}
            </span>
          );
        },
        sortingFn: (a, b) => {
          const aVal = parseFloat(
            duration === '24h'
              ? a.original.pnl_24h
              : duration === '7d'
                ? a.original.pnl_7d
                : a.original.pnl_30d
          );
          const bVal = parseFloat(
            duration === '24h'
              ? b.original.pnl_24h
              : duration === '7d'
                ? b.original.pnl_7d
                : b.original.pnl_30d
          );
          return aVal - bVal;
        }
      },
      {
        accessorKey: 'win_rate',
        header: `Win Rate ${duration}`,
        cell: ({ row }) =>
          formatPercent(
            duration === '24h'
              ? row.original.win_rate_24h
              : duration === '7d'
                ? row.original.win_rate_7d
                : row.original.win_rate_30d
          ),
        enableSorting: false
      },
      {
        accessorKey: 'volume',
        header: `${duration} Volume`,
        cell: ({ row }) =>
          formatNumberShort(
            duration === '24h'
              ? row.original.volume_24h
              : duration === '7d'
                ? row.original.volume_7d
                : row.original.volume_30d
          ),
        sortingFn: (a, b) => {
          const aVal = parseFloat(
            duration === '24h'
              ? a.original.volume_24h
              : duration === '7d'
                ? a.original.volume_7d
                : a.original.volume_30d
          );
          const bVal = parseFloat(
            duration === '24h'
              ? b.original.volume_24h
              : duration === '7d'
                ? b.original.volume_7d
                : b.original.volume_30d
          );
          return aVal - bVal;
        }
      },
      {
        accessorKey: 'trade_count_24h',
        header: 'Trades 24h',
        cell: ({ row }) => row.original.trade_count_24h.toLocaleString(),
        enableSorting: false
      },
      {
        accessorKey: 'avg_trade_size',
        header: 'Avg Trade',
        cell: ({ row }) => formatNumberShort(row.original.avg_trade_size || '0'),
        enableSorting: false
      }
    ],
    [duration]
  );

  const filteredData = useMemo(() => {
    return allWhales.filter((whale) => {
      if (whale.broker_id === 'orderly') return false;
      if (duration === '24h') {
        return whale.trade_count_24h > 0;
      } else if (duration === '7d') {
        return parseFloat(whale.volume_7d) > 0;
      } else {
        return parseFloat(whale.volume_30d) > 0;
      }
    });
  }, [allWhales, duration]);

  const table = useReactTable<TopAddressEntry>({
    data: filteredData,
    columns,
    state: { sorting },
    onSortingChange: (updater) => {
      const newSorting = typeof updater === 'function' ? updater(sorting) : updater;
      setSorting(newSorting);
    },
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel()
  });

  if (error) {
    return (
      <div className="flex flex-col gap-4 flex-items-center [&>*]:w-full [&>*]:max-w-full lg:max-w-[50rem] px-2 sm:px-0">
        <h2 className="mb-2 text-lg sm:text-xl">Whale Tracker</h2>
        <div className="text-red-500">Error: {error.message}</div>
      </div>
    );
  }

  return (
    <div className="space-y-4 sm:space-y-8 animate-fade-in flex flex-col align-center">
      <div className="text-center space-y-2 sm:space-y-4">
        <h2 className="text-2xl font-bold text-white">Whale Tracker</h2>
        <p className="text-gray-300 max-w-3xl mx-auto">
          Track top traders by PnL, volume, and win rate across different time windows. Click the
          eye icon to view a whale&apos;s positions and recent trades.
        </p>
      </div>

      <div
        className="card w-full space-y-4 sm:space-y-6"
        style={{ background: '#130E1D', border: 'none' }}
      >
        <div className="flex gap-2 justify-center">
          {(['24h', '7d', '30d'] as const).map((d) => {
            const isActive = duration === d;
            return (
              <button
                key={d}
                onClick={() => setDuration(d)}
                className="py-2 px-6 rounded-full text-[13px] font-semibold cursor-pointer transition-all duration-150"
                style={{
                  border: 'none',
                  background: isActive ? '#6700CE' : '#221E30',
                  color: isActive ? '#fff' : 'rgba(255,255,255,0.45)'
                }}
              >
                {d}
              </button>
            );
          })}
        </div>

        <div className="w-full">
          {isLoading ? (
            <div className="flex justify-center py-12 w-full">
              <Spinner size="2.5rem" />
            </div>
          ) : (
            <div className="space-y-2">
              <div className="flex justify-between items-center px-1">
                <span className="text-sm text-gray-400">
                  {filteredData.length} whales
                  {lastUpdated > 0 && (
                    <span className="ml-2">
                      · Updated {new Date(lastUpdated).toLocaleTimeString()}
                    </span>
                  )}
                </span>
              </div>

              <div className="w-full overflow-x-auto relative">
                {isLoading && (
                  <div className="absolute inset-0 bg-bg-overlay flex items-center justify-center z-10 rounded-xl">
                    <Spinner size="2rem" />
                  </div>
                )}
                <Table.Root className="w-full">
                  <Table.Header>
                    {table.getHeaderGroups().map((headerGroup) => (
                      <Table.Row key={headerGroup.id} className="border-b border-border-primary">
                        {headerGroup.headers.map((header) => (
                          <Table.ColumnHeaderCell
                            key={header.id}
                            colSpan={header.colSpan}
                            className="py-2 px-2 sm:py-4 sm:px-3"
                          >
                            {header.isPlaceholder ? null : (
                              <div
                                className={
                                  header.column.getCanSort()
                                    ? 'cursor-pointer select-none hover:text-primary-light transition-colors duration-150 text-sm font-medium'
                                    : 'text-sm font-medium'
                                }
                                onClick={header.column.getToggleSortingHandler()}
                                onKeyDown={(ev) => {
                                  if (ev.key === 'Enter') {
                                    header.column.getToggleSortingHandler();
                                  }
                                }}
                                role="button"
                                tabIndex={0}
                              >
                                {flexRender(header.column.columnDef.header, header.getContext())}
                                {{
                                  asc: ' 🔼',
                                  desc: ' 🔽'
                                }[header.column.getIsSorted() as string] ?? null}
                              </div>
                            )}
                          </Table.ColumnHeaderCell>
                        ))}
                      </Table.Row>
                    ))}
                  </Table.Header>

                  <Table.Body>
                    {table.getRowModel().rows.map((row, index) => (
                      <Table.Row
                        key={row.id}
                        className={`border-b border-border-primary hover:bg-bg-tertiary transition-colors duration-150 ${
                          index % 2 === 0 ? 'bg-bg-secondary' : 'bg-bg-primary'
                        }`}
                      >
                        {row.getVisibleCells().map((cell) => (
                          <Table.Cell
                            key={cell.id}
                            className="align-middle text-sm py-2 px-2 sm:py-3 sm:px-3"
                          >
                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                          </Table.Cell>
                        ))}
                      </Table.Row>
                    ))}
                  </Table.Body>
                </Table.Root>
              </div>
            </div>
          )}
        </div>
      </div>

      {selectedWhale && (
        <WhaleDetailModal
          open={!!selectedWhale}
          onOpenChange={(open) => !open && setSelectedWhale(null)}
          address={selectedWhale.address}
          brokerId={selectedWhale.brokerId}
        />
      )}
    </div>
  );
};
