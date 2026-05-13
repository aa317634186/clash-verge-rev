import {
  Button,
  List,
  ListItem,
  ListItemText,
  ListSubheader,
  MenuItem,
  Select,
  Switch,
} from "@mui/material";
import type { SelectChangeEvent } from "@mui/material";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import {
  getSplitTunnelApps,
  requestBatteryOptExclusion,
  setAutoStartOnBoot,
  setSplitTunnelMode,
} from "@/services/android-api";

const AndroidVpnSettings = () => {
  const { t } = useTranslation();
  const [autoStart, setAutoStart] = useState(false);
  const [splitMode, setSplitMode] = useState("disabled");
  const [appCount, setAppCount] = useState(0);

  useEffect(() => {
    getSplitTunnelApps()
      .then((apps) => setAppCount(apps.length))
      .catch(() => {});
  }, []);

  const handleAutoStartChange = useCallback(
    async (_: React.ChangeEvent<HTMLInputElement>, checked: boolean) => {
      try {
        await setAutoStartOnBoot(checked);
        setAutoStart(checked);
      } catch {
        // revert on error
      }
    },
    [],
  );

  const handleSplitModeChange = useCallback(
    async (e: SelectChangeEvent<string>) => {
      const mode = e.target.value;
      try {
        await setSplitTunnelMode(mode);
        setSplitMode(mode);
      } catch {
        // revert on error
      }
    },
    [],
  );

  const handleBatteryExclusion = useCallback(async () => {
    try {
      await requestBatteryOptExclusion();
    } catch {
      // handle error
    }
  }, []);

  return (
    <List
      subheader={
        <ListSubheader disableSticky>
          {t("settings.sections.androidVpn.title")}
        </ListSubheader>
      }
    >
      <ListItem>
        <ListItemText
          primary={t("settings.sections.androidVpn.autoStart")}
          secondary={t("settings.sections.androidVpn.autoStartDesc")}
        />
        <Switch
          edge="end"
          checked={autoStart}
          onChange={handleAutoStartChange}
        />
      </ListItem>

      <ListItem>
        <ListItemText
          primary={t("settings.sections.androidVpn.batteryOptimization")}
          secondary={t("settings.sections.androidVpn.batteryOptimizationDesc")}
        />
        <Button
          variant="outlined"
          size="small"
          onClick={handleBatteryExclusion}
        >
          {t("settings.sections.androidVpn.request")}
        </Button>
      </ListItem>

      <ListItem>
        <ListItemText
          primary={t("settings.sections.androidVpn.splitTunneling")}
          secondary={t("settings.sections.androidVpn.splitTunnelingDesc")}
        />
        <Select size="small" value={splitMode} onChange={handleSplitModeChange}>
          <MenuItem value="disabled">
            {t("settings.sections.androidVpn.splitModes.disabled")}
          </MenuItem>
          <MenuItem value="whitelist">
            {t("settings.sections.androidVpn.splitModes.whitelist")}
          </MenuItem>
          <MenuItem value="blacklist">
            {t("settings.sections.androidVpn.splitModes.blacklist")}
          </MenuItem>
        </Select>
      </ListItem>

      {splitMode !== "disabled" && (
        <ListItem>
          <ListItemText
            primary={t("settings.sections.androidVpn.perAppList")}
            secondary={t("settings.sections.androidVpn.perAppListDesc", {
              count: appCount,
            })}
          />
        </ListItem>
      )}
    </List>
  );
};

export default AndroidVpnSettings;
