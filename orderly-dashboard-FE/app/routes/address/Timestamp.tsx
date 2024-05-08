import { Button } from '@radix-ui/themes';
import dayjs from 'dayjs';
import { FC, useState } from 'react';

export const Timestamp: FC<{ timestamp?: number }> = ({ timestamp }) => {
  const [humanReadable, setHumanReadable] = useState(true);

  if (!timestamp) return '';
  return (
    <Button
      className="p-1 h-auto bg-[--accent-5] hover:bg-[--accent-4]"
      onClick={() => {
        setHumanReadable(!humanReadable);
      }}
    >
      {humanReadable ? dayjs(timestamp * 1_000).format('lll') : timestamp}
    </Button>
  );
};
