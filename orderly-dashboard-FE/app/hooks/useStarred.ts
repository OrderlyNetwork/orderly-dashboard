import { useCallback, useEffect, useState } from 'react';

export type StarredItem = {
  id: string;
  type: 'dashboard' | 'query';
  title: string;
  description: string;
  starredAt: number; // unix ms
};

const STORAGE_KEY = 'orderly_analytics_starred';

function load(): StarredItem[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? (JSON.parse(raw) as StarredItem[]) : [];
  } catch {
    return [];
  }
}

function save(items: StarredItem[]) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(items));
}

export function useStarred() {
  const [starred, setStarred] = useState<StarredItem[]>([]);

  useEffect(() => {
    setStarred(load());
  }, []);

  const isStarred = useCallback((id: string) => starred.some((s) => s.id === id), [starred]);

  const toggleStar = useCallback((item: Omit<StarredItem, 'starredAt'>) => {
    setStarred((prev) => {
      const exists = prev.some((s) => s.id === item.id);
      const next = exists
        ? prev.filter((s) => s.id !== item.id)
        : [...prev, { ...item, starredAt: Date.now() }];
      save(next);
      return next;
    });
  }, []);

  const removeStar = useCallback((id: string) => {
    setStarred((prev) => {
      const next = prev.filter((s) => s.id !== id);
      save(next);
      return next;
    });
  }, []);

  return { starred, isStarred, toggleStar, removeStar };
}
