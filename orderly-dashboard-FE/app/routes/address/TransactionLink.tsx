import { FC } from 'react';

import { Shortened } from './Shortened';

export const TransactionLink: FC<{ value?: number | string; displayCount?: number }> = ({
  value,
  displayCount
}) => {
  if (value == null) return null;

  const txId = String(value);
  const explorerUrl = `https://explorer.orderly.network/tx/${txId}`;

  return (
    <a href={explorerUrl} target="_blank" rel="noopener noreferrer" className="hover:underline">
      <Shortened value={txId} displayCount={displayCount} />
    </a>
  );
};
