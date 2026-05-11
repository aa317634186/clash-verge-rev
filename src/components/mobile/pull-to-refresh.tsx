import { Box, CircularProgress } from "@mui/material";
import { useCallback, useRef, useState } from "react";
import type { ReactNode, TouchEvent } from "react";

interface PullToRefreshProps {
  onRefresh: () => Promise<void>;
  children: ReactNode;
  disabled?: boolean;
}

const THRESHOLD = 80;

const PullToRefresh = ({
  onRefresh,
  children,
  disabled,
}: PullToRefreshProps) => {
  const [pulling, setPulling] = useState(false);
  const [refreshing, setRefreshing] = useState(false);
  const [pullDistance, setPullDistance] = useState(0);
  const startY = useRef(0);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleTouchStart = useCallback(
    (e: TouchEvent) => {
      if (disabled || refreshing) return;
      const container = containerRef.current;
      if (container && container.scrollTop === 0) {
        startY.current = e.touches[0].clientY;
        setPulling(true);
      }
    },
    [disabled, refreshing],
  );

  const handleTouchMove = useCallback(
    (e: TouchEvent) => {
      if (!pulling || disabled || refreshing) return;
      const currentY = e.touches[0].clientY;
      const distance = Math.max(0, currentY - startY.current);
      setPullDistance(Math.min(distance, THRESHOLD * 1.5));
    },
    [pulling, disabled, refreshing],
  );

  const handleTouchEnd = useCallback(async () => {
    if (!pulling || disabled) return;
    setPulling(false);

    if (pullDistance >= THRESHOLD) {
      setRefreshing(true);
      try {
        await onRefresh();
      } finally {
        setRefreshing(false);
      }
    }
    setPullDistance(0);
  }, [pulling, pullDistance, disabled, onRefresh]);

  return (
    <Box
      ref={containerRef}
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
      sx={{ position: "relative", overflow: "auto", height: "100%" }}
    >
      {(pullDistance > 0 || refreshing) && (
        <Box
          sx={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            height: refreshing ? 48 : pullDistance * 0.5,
            transition: pulling ? "none" : "height 0.2s ease",
            overflow: "hidden",
          }}
        >
          <CircularProgress
            size={24}
            variant={refreshing ? "indeterminate" : "determinate"}
            value={refreshing ? undefined : (pullDistance / THRESHOLD) * 100}
          />
        </Box>
      )}
      {children}
    </Box>
  );
};

export default PullToRefresh;
