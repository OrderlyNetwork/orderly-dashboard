import { Button } from '@radix-ui/themes';
import dayjs from 'dayjs';
import { FC, useState } from 'react';

export const Timestamp: FC<{ timestamp?: number; multiplier?: number }> = ({
  timestamp,
  multiplier
}) => {
  const [humanReadable, setHumanReadable] = useState(true);

  if (!timestamp) return '';
  return (
    <Button
      className="p-1 h-auto bg-[--accent-5] hover:bg-[--accent-4]"
      onClick={() => {
        setHumanReadable(!humanReadable);
      }}
    >
      {humanReadable ? dayjs(timestamp * (multiplier ?? 1)).format('lll') : timestamp}
    </Button>
  );
};
