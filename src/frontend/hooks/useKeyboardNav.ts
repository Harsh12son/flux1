import { useCallback, useState } from "react";

export const useKeyboardNav = (itemCount: number) => {
  const [activeIndex, setActiveIndex] = useState(0);

  const moveUp = useCallback(() => {
    setActiveIndex((prev) => (itemCount === 0 ? 0 : (prev - 1 + itemCount) % itemCount));
  }, [itemCount]);

  const moveDown = useCallback(() => {
    setActiveIndex((prev) => (itemCount === 0 ? 0 : (prev + 1) % itemCount));
  }, [itemCount]);

  const reset = useCallback(() => setActiveIndex(0), []);

  return { activeIndex, moveUp, moveDown, reset, setActiveIndex };
};

