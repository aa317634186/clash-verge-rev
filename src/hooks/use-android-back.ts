import { useEffect } from "react";
import { useLocation, useNavigate } from "react-router";

import { isAndroid } from "@/utils/platform";

/**
 * Hook that handles Android hardware back button.
 * Navigates back if not at root, otherwise does nothing.
 */
export function useAndroidBack() {
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    if (!isAndroid()) return;

    const handlePopState = () => {
      if (location.pathname !== "/") {
        navigate(-1);
      }
    };

    window.addEventListener("popstate", handlePopState);
    return () => {
      window.removeEventListener("popstate", handlePopState);
    };
  }, [navigate, location.pathname]);
}
