import { useState } from 'react';
import { Info } from 'react-feather';
import { getSettings, updateSettings } from '../state/settingsState';
import { useTranslation } from '../utils/i18n/useTranslation';

const SettingsMenu = () => {
    const t = useTranslation();
    const [processWatcherPollRate, setProcessWatcherPollRate] =
        useState<string>(String(getSettings().pollRate));

    const handlePollRateBlur = async () => {
        if (isNaN(Number(processWatcherPollRate))) {
            setProcessWatcherPollRate(String(getSettings().pollRate));
            return;
        }

        if (Number(processWatcherPollRate) === getSettings().pollRate) return;

        updateSettings({ pollRate: Number(processWatcherPollRate) });
    };

    const handleCheckForUpdatesToggle = (
        e: React.ChangeEvent<HTMLInputElement>
    ) => {
        updateSettings({ checkForUpdates: e.target.checked });
    };

    return (
        <div className="sidebar bg-base-100 drop-shadow-2xl flex flex-col p-3 pt-16">
            <div className="form-control w-full">
                <label className="label pb-1">
                    <span className="text-2xs uppercase font-semibold">
                        {t('settings.checkForUpdates.title')}
                    </span>
                </label>
                <input
                    id="updates_on_launch"
                    type="checkbox"
                    className="toggle toggle-accent toggle-md"
                    checked={getSettings().checkForUpdates}
                    onChange={handleCheckForUpdatesToggle}
                />
            </div>
            <div className="divider mt-2 mb-1"></div>
            <div className="form-control w-full mb-4">
                <label
                    htmlFor="pollRate"
                    className="label pb-1 justify-start gap-2"
                >
                    <span className="text-2xs uppercase font-semibold">
                        {t('settings.processPollRate.title')}
                    </span>
                    <div
                        className="tooltip before:w-[300px] before:-left-[10px] before:translate-x-0"
                        data-tip={t('settings.processPollRate.description')}
                    >
                        <Info size=".8em" />
                    </div>
                </label>
                <input
                    type="number"
                    id="pollRate"
                    className="input input-bordered w-full"
                    value={processWatcherPollRate}
                    onChange={(e) => setProcessWatcherPollRate(e.target.value)}
                    onBlur={handlePollRateBlur}
                />
            </div>
        </div>
    );
};

export default SettingsMenu;
