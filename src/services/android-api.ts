import { invoke } from "@tauri-apps/api/core";

export async function startVpn(configPath: string): Promise<void> {
  return invoke("start_vpn", { configPath });
}

export async function stopVpn(): Promise<void> {
  return invoke("stop_vpn");
}

export async function getVpnStatus(): Promise<boolean> {
  return invoke("get_vpn_status");
}

export async function requestVpnPermission(): Promise<void> {
  return invoke("request_vpn_permission");
}

export async function getSplitTunnelApps(): Promise<string[]> {
  return invoke("get_split_tunnel_apps");
}

export async function setSplitTunnelApps(apps: string[]): Promise<void> {
  return invoke("set_split_tunnel_apps", { apps });
}

export async function setSplitTunnelMode(mode: string): Promise<void> {
  return invoke("set_split_tunnel_mode", { mode });
}

export async function requestBatteryOptExclusion(): Promise<void> {
  return invoke("request_battery_optimization_exclusion");
}

export async function setAutoStartOnBoot(enabled: boolean): Promise<void> {
  return invoke("set_auto_start_on_boot", { enabled });
}
