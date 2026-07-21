import { Tooltip } from '@radix-ui/themes';
import { FC } from 'react';

export type BrokerBadgeProps = {
  broker: string | null;
  className?: string;
};

/**
 * Small pill indicating the listing broker of a permissionless market.
 * Renders nothing for canonical (no-broker) symbols so most call sites are
 * no-ops. Tooltip explains the suffix for users unfamiliar with the format.
 */
export const BrokerBadge: FC<BrokerBadgeProps> = ({ broker, className }) => {
  if (!broker) return null;
  return (
    <Tooltip content={`Permissionless market listed by broker "${broker}"`}>
      <span
        className={`inline-flex items-center rounded px-1.5 py-0.5 text-[10px] font-medium leading-none ${className ?? ''}`}
        style={{
          background: 'rgba(156,117,255,0.15)',
          color: '#D4B2FF',
          border: '1px solid rgba(156,117,255,0.25)'
        }}
      >
        {broker}
      </span>
    </Tooltip>
  );
};
