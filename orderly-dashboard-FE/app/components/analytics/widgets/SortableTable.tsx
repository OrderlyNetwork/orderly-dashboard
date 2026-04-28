import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable
} from '@tanstack/react-table';
import { FC, useState } from 'react';

type SortableTableProps<T extends object> = {
  data: T[];
  columns: ColumnDef<T, unknown>[];
  rowKey?: (row: T) => string;
};

function SortIcon({ asc }: { asc: boolean | null }) {
  if (asc === null)
    return (
      <svg width="10" height="14" viewBox="0 0 10 14" fill="none" style={{ opacity: 0.35 }}>
        <path d="M5 1L1 5h8L5 1zM5 13l4-4H1l4 4z" fill="currentColor" />
      </svg>
    );
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none" style={{ opacity: 0.9 }}>
      {asc ? (
        <path d="M5 1L1 7h8L5 1z" fill="#9C75FF" />
      ) : (
        <path d="M5 9L9 3H1l4 6z" fill="#9C75FF" />
      )}
    </svg>
  );
}

export function SortableTable<T extends object>({ data, columns, rowKey }: SortableTableProps<T>) {
  const [sorting, setSorting] = useState<SortingState>([]);

  const table = useReactTable({
    data,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel()
  });

  return (
    <div style={{ overflowX: 'auto', width: '100%' }}>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
        <thead>
          {table.getHeaderGroups().map((hg) => (
            <tr key={hg.id}>
              {hg.headers.map((header) => {
                const canSort = header.column.getCanSort();
                const sortDir = header.column.getIsSorted();
                return (
                  <th
                    key={header.id}
                    onClick={canSort ? header.column.getToggleSortingHandler() : undefined}
                    style={{
                      padding: '10px 14px',
                      textAlign: 'left',
                      fontSize: 11,
                      fontWeight: 600,
                      color: 'rgba(255,255,255,0.45)',
                      textTransform: 'uppercase',
                      letterSpacing: '0.07em',
                      borderBottom: '1px solid rgba(156,117,255,0.12)',
                      cursor: canSort ? 'pointer' : 'default',
                      whiteSpace: 'nowrap',
                      userSelect: 'none'
                    }}
                  >
                    <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                      {flexRender(header.column.columnDef.header, header.getContext())}
                      {canSort && (
                        <SortIcon
                          asc={sortDir === 'asc' ? true : sortDir === 'desc' ? false : null}
                        />
                      )}
                    </div>
                  </th>
                );
              })}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row, i) => (
            <tr
              key={rowKey ? rowKey(row.original) : row.id}
              style={{
                background: i % 2 === 0 ? 'transparent' : 'rgba(156,117,255,0.03)',
                transition: 'background 0.15s'
              }}
              onMouseEnter={(e) => {
                (e.currentTarget as HTMLTableRowElement).style.background =
                  'rgba(156,117,255,0.08)';
              }}
              onMouseLeave={(e) => {
                (e.currentTarget as HTMLTableRowElement).style.background =
                  i % 2 === 0 ? 'transparent' : 'rgba(156,117,255,0.03)';
              }}
            >
              {row.getVisibleCells().map((cell) => (
                <td
                  key={cell.id}
                  style={{
                    padding: '11px 14px',
                    color: 'rgba(255,255,255,0.85)',
                    borderBottom: '1px solid rgba(156,117,255,0.07)',
                    whiteSpace: 'nowrap'
                  }}
                >
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      {data.length === 0 && (
        <div
          style={{
            textAlign: 'center',
            padding: '40px 0',
            color: 'rgba(255,255,255,0.3)',
            fontSize: 14
          }}
        >
          No data available
        </div>
      )}
    </div>
  );
}

// Make it usable as FC too
export const SortableTableFC = SortableTable as FC<SortableTableProps<object>>;
