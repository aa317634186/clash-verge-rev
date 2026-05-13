import { BottomNavigation, BottomNavigationAction, Paper } from "@mui/material";
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate } from "react-router";

import { navItems } from "@/pages/_routers";

/** Primary tabs to show on mobile bottom navigation. */
const MOBILE_TABS = ["/", "/proxies", "/profile", "/connections", "/settings"];

const MobileNavBar = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();

  const currentPath = location.pathname;
  const activeIndex = MOBILE_TABS.indexOf(currentPath);
  const value = activeIndex >= 0 ? activeIndex : 0;

  const items = MOBILE_TABS.map((path) => {
    const item = navItems.find((n) => n.path === path);
    return item!;
  });

  return (
    <Paper className="mobile-bottom-nav" elevation={3} square>
      <BottomNavigation
        showLabels
        value={value}
        onChange={(_event, newValue) => {
          const target = items[newValue];
          if (target) {
            navigate(target.path);
          }
        }}
      >
        {items.map((item) => (
          <BottomNavigationAction
            key={item.path}
            label={t(item.label)}
            icon={item.icon[0]}
          />
        ))}
      </BottomNavigation>
    </Paper>
  );
};

export default MobileNavBar;
