import Snackbar from '@mui/material/Snackbar';
import { ClipboardCopyIcon } from '@radix-ui/react-icons';
import { Tooltip } from '@radix-ui/themes';
import { FC, useState } from 'react';

export const Shortened: FC<{ value?: number | string; displayCount?: number }> = ({
  value,
  displayCount
}) => {
  const [open, setOpen] = useState(false);

  if (value == null) return;
  value = String(value);
  displayCount = displayCount ?? 4;
  return (
    <div className="flex flex-items-center gap-2">
      <Tooltip content={`${value}`}>
        <span>
          {value.substring(0, displayCount)}...{value.substr(-displayCount)}
        </span>
      </Tooltip>
      <ClipboardCopyIcon
        className="hover:cursor-pointer"
        onClick={() => {
          navigator.clipboard.writeText(value);
          setOpen(true);
        }}
      />
      <Snackbar
        open={open}
        autoHideDuration={3_000}
        onClose={(_event, reason?: string) => {
          if (reason === 'clickaway') {
            return;
          }
          setOpen(false);
        }}
        message="Copied to clipboard"
      />
    </div>
  );
};
