import { FC } from 'react';

export const TransactionLink: FC<{
  value?: number | string;
  displayCount?: number;
}> = ({ value, displayCount = 6 }) => {
  if (value == null) return null;

  const txId = String(value);
  const explorerUrl = `https://explorer.orderly.network/tx/${txId}`;

  return (
    <a href={explorerUrl} target="_blank" rel="noopener noreferrer" className="hover:underline">
      <span>
        {txId.substring(0, displayCount)}...{txId.substr(-4)}
      </span>
    </a>
  );
};
