import { Tooltip } from '@radix-ui/themes';
import { FC } from 'react';

export const Shortened: FC<{ value?: number | string; displayCount?: number }> = ({
  value,
  displayCount
}) => {
  if (value == null) return;
  value = String(value);
  displayCount = displayCount ?? 4;
  return (
    <Tooltip content={`${value}`}>
      <span>
        {value.substring(0, displayCount)}...{value.substr(-displayCount)}
      </span>
    </Tooltip>
  );
};
