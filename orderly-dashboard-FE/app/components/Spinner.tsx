import { FC } from 'react';

export const Spinner: FC<{
  size?: string;
  overlay?: boolean;
  inline?: boolean;
  className?: string;
}> = ({ size, overlay, inline, className }) => {
  return (
    <div
      className={`flex flex-items-center flex-justify-center ${inline === true ? '' : 'w-full'} ${overlay ? 'absolute p-1 h-full top-0' : ''} max-h-full ${className ?? ''}`}
    >
      <div
        className={`${overlay ? 'h-full' : 'w-5 h-5'} aspect-square max-w-full max-h-full b-rd-36 aspect-square b-dotted b-4 b-transparent b-t-white b-r-white`}
        style={{
          width: size ?? undefined,
          height: size ?? undefined,
          animation: 'rotate 1200ms linear infinite'
        }}
      />
    </div>
  );
};
