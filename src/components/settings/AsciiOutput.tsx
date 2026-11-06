import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface AsciiOutputProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

export const AsciiOutput: React.FC<AsciiOutputProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();

    const enabled = getSetting("ascii_output") ?? false;

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={(enabled) => updateSetting("ascii_output", enabled)}
        isUpdating={isUpdating("ascii_output")}
        label={t("settings.advanced.asciiOutput.label")}
        description={t("settings.advanced.asciiOutput.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  },
);
