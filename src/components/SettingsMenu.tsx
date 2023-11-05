import { useState } from "react";
import { Info } from "react-feather"
import Settings from "../types/SettingsType";

interface Props {
    settings: Settings;
    updateSettings: (settings: Settings) => void;
}

const SettingsMenu = ({settings, updateSettings}: Props) => {
    const [processWatcherPollRate, setProcessWatcherPollRate] = useState<string>(String(settings.pollRate));

    const handlePollRateBlur = async () => {
        if(isNaN(Number(processWatcherPollRate))) {
            setProcessWatcherPollRate(String(settings.pollRate));
            return;
        }

        // If the number hasnt changed then dont update
        if(Number(processWatcherPollRate) === settings.pollRate) return;

        updateSettings({...settings, pollRate: Number(processWatcherPollRate)});
    }
    
    return (
        <div className="sidebar bg-base-100 drop-shadow-2xl flex flex-col p-3 pt-16">
            <div className="form-control w-full">
                <label className="label pb-1">
                    <span className="text-2xs uppercase font-semibold">
                        Check for updates on launch
                    </span>
                </label>
                <input id="updates_on_launch" type="checkbox" className="toggle toggle-accent toggle-md" checked={settings.checkForUpdates} onChange={e => updateSettings({...settings, checkForUpdates: e.target.checked})
                    }/>  
            </div>
            <div className="divider mt-2 mb-1"></div>
            <div className="form-control w-full mb-4">
                <label htmlFor="pollRate" className="label pb-1 justify-start gap-2">
                    <span className="text-2xs uppercase font-semibold">Process Poll Rate (ms)</span>
                    <div className="tooltip before:w-[300px] before:-left-[10px] before:translate-x-0" data-tip="How often should we check for new applicaitons being launched?">
                        <Info size=".8em"/>
                    </div>
                    
                </label>
                <input 
                    type="number" 
                    id="pollRate" 
                    className="input input-bordered w-full" 
                    value={processWatcherPollRate} 
                    onChange={e => setProcessWatcherPollRate(e.target.value)}
                    onBlur={handlePollRateBlur}
                />
            </div>
        </div>
    )
}

export default SettingsMenu;