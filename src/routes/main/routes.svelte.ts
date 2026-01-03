/**
 * @file This file defines the tool routes for the toolbox sidebar.
 */
import ActivityIcon from "virtual:icons/lucide/activity";
import CalculatorIcon from "virtual:icons/lucide/calculator";
import HourglassIcon from "virtual:icons/lucide/hourglass";
import UploadIcon from "virtual:icons/lucide/upload-cloud";
import PaletteIcon from "virtual:icons/lucide/palette";
import SettingsIcon from "virtual:icons/lucide/settings";

// Tool-level routes for the left sidebar
export const TOOL_ROUTES = {
  "/main/dps": { label: "DPS检测", icon: ActivityIcon },
  "/main/module-calc": { label: "模组计算", icon: CalculatorIcon },
};

// Sub-routes for DPS tool (tabs in the right panel)
export const DPS_SUB_ROUTES = {
  "/main/dps/history": { label: "历史", icon: HourglassIcon },
  // "/main/dps/uploading": { label: "Uploading", icon: UploadIcon },
  "/main/dps/themes": { label: "主题", icon: PaletteIcon },
  "/main/dps/settings": { label: "设置", icon: SettingsIcon },
};

// Legacy export for backward compatibility (if needed)
export const SIDEBAR_ROUTES = DPS_SUB_ROUTES;
