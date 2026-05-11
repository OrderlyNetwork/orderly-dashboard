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
    <div className="overflow-x-auto w-full">
      <table className="w-full border-collapse text-sm">
        <thead>
          {table.getHeaderGroups().map((hg) => (
            <tr key={hg.id}>
              {hg.headers.map((header, colIdx) => {
                const canSort = header.column.getCanSort();
                const sortDir = header.column.getIsSorted();
                const isFirstCol = colIdx === 0;
                return (
                  <th
                    key={header.id}
                    onClick={canSort ? header.column.getToggleSortingHandler() : undefined}
                    className={`py-[10px] px-[14px] text-left text-[11px] font-semibold text-[rgba(255,255,255,0.45)] uppercase tracking-[0.07em] whitespace-nowrap select-none ${canSort ? 'cursor-pointer' : ''} sticky top-0 bg-[rgba(20,15,35,.95)] ${isFirstCol ? 'left-0 z-3' : 'z-2'}`}
                    style={{ borderBottom: '1px solid rgba(156,117,255,0.12)' }}
                  >
                    <div className="flex items-center gap-1.5">
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
          {table.getRowModel().rows.map((row, i) => {
            const bgClass = i % 2 === 0 ? '' : 'bg-[rgba(156,117,255,0.03)]';
            return (
              <tr
                key={rowKey ? rowKey(row.original) : row.id}
                className={`transition-all duration-150 ${bgClass}`}
                onMouseEnter={(e) => {
                  (e.currentTarget as HTMLTableRowElement).style.background =
                    'rgba(156,117,255,0.08)';
                }}
                onMouseLeave={(e) => {
                  (e.currentTarget as HTMLTableRowElement).style.background =
                    i % 2 === 0 ? 'transparent' : 'rgba(156,117,255,0.03)';
                }}
              >
                {row.getVisibleCells().map((cell, colIdx) => (
                  <td
                    key={cell.id}
                    className={`py-[11px] px-[14px] text-[rgba(255,255,255,0.85)] whitespace-nowrap ${colIdx === 0 ? 'sticky left-0 z-1' : ''}`}
                    style={{
                      borderBottom: '1px solid rgba(156,117,255,0.07)',
                      ...(colIdx === 0
                        ? { background: i % 2 === 0 ? 'rgba(20,15,35,.95)' : 'rgba(22,17,40,.95)' }
                        : {})
                    }}
                  >
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            );
          })}
        </tbody>
      </table>
      {data.length === 0 && (
        <div className="text-center py-10 text-[rgba(255,255,255,0.3)] text-sm">
          No data available
        </div>
      )}
    </div>
  );
}

export const SortableTableFC = SortableTable as FC<SortableTableProps<object>>;
