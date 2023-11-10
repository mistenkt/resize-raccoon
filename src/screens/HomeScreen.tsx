import { useState } from 'react';
import ProfileListItem from '../components/ProfileListItem';
import { Plus, Settings as SettingsIcon } from 'react-feather';
import SettingsMenu from '../components/SettingsMenu';
import { getSettings, updateSettings } from '../state/settingsState';
import { getAppVersion } from '../state/appVersionState';
import { getProfiles } from '../state/profileState';
import { setScreen } from '../state/screenState';
import { Screen } from '../types/ScreenTypes';

const HomeScreen = () => {
    const [settingsOpen, setSettingsOpen] = useState(false);

    const handleProcessWatcherToggle = (
        e: React.ChangeEvent<HTMLInputElement>
    ) => {
        updateSettings({ processWatcherEnabled: e.target.checked });
    };

    const handleNewProfile = () => {
        setScreen(Screen.PROFILE_EDITOR);
    };

    return (
        <div className="relative">
            <button
                className="btn btn-circle btn-outline btn-sm fixed top-4 left-4 z-[5]"
                onClick={() => setSettingsOpen(!settingsOpen)}
            >
                <SettingsIcon size={16} />
            </button>
            <div
                className={`w-full h-screen settings-drawer ${
                    settingsOpen ? 'sidebar-open' : ''
                }`}
            >
                <SettingsMenu />
                <div className="home-screen-grid grid h-screen grid-cols-[45%_55%] w-full bg-gradient-to-t from-[#660e99] to-[#941882]">
                    <div className="logo-col h-full flex flex-col justify-center items-center pb-8 pr-8 pl-8 pt-8">
                        <img
                            src="./resize-raccoon-logo.png"
                            className="w-full max-w-full h-auto object-contain max-h-[50vh]"
                        />
                        <div className="text-container mt-4">
                            <span className="text-element text-4xl font-bold text-slate-200 tracking-wide font-noto">
                                ResizeRaccoon
                            </span>
                        </div>
                        <div className="only-home flex flex-col">
                            <div className="uppercase text-2xs font-bold tracking-wide">
                                <div className="version-info flex justify-center gap-1">
                                    <div>
                                        <span className="lowercase">v</span>
                                        {getAppVersion()}
                                    </div>
                                    <div>•</div>
                                    <a
                                        className="link"
                                        target="_blank"
                                        href="https://github.com/mistenkt/resize-raccoon"
                                    >
                                        Homepage
                                    </a>
                                </div>
                            </div>
                            <div className="divider w-20 self-center mt-2 mb-2" />
                            <div className="form-control flex w-full auto-resize-toggle">
                                <div
                                    className="tooltip tooltip-bottom self-center"
                                    data-tip="Process watching will automatically apply a profile when the application is started"
                                >
                                    <label className="label cursor-pointer justify-end gap-2 items-center p-0">
                                        <span className="label-text">
                                            <span className="badge font-semibold tracking-wide badge-outline badge-xs text-2xs uppercase inline mr-1">
                                                beta
                                            </span>
                                            Process watcher
                                        </span>
                                        <input
                                            type="checkbox"
                                            className="toggle toggle-accent toggle-md"
                                            checked={
                                                getSettings()
                                                    .processWatcherEnabled
                                            }
                                            onChange={
                                                handleProcessWatcherToggle
                                            }
                                        />
                                    </label>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="flex flex-col h-full overflow-y-auto pr-8 pt-12 pb-12 w-full">
                        {getProfiles().map((p, i) => (
                            <ProfileListItem key={i} profile={p} />
                        ))}
                    </div>
                </div>
                <button
                    className="btn btn-square btn-sm btn-outline fixed bottom-4 right-4 z-[5]"
                    onClick={handleNewProfile}
                >
                    <Plus size={20} />
                </button>
            </div>
        </div>
    );
};

export default HomeScreen;
