import { useEffect, useState } from "react";
import { Profile } from "../types/ProfileTypes";
import backend from "../utils/backend";
import ProfileListItem from "../components/ProfileListItem";
import { PlusCircle, Settings as SettingsIcon } from "react-feather";
import SettingsMenu from "../components/SettingsMenu";
import Settings from "../types/SettingsType";

interface Props {
    onNewProfile: () => void;
    onEditProfile: (profile: Profile) => void;
    version: string;
    settings: Settings;
    updateSettings: (settings: Settings) => void;
}

const HomeScreen = ({ onEditProfile, onNewProfile, version, settings, updateSettings}: Props) => {
    const [profiles, setProfiles] = useState<Profile[]>([]);
    const [settingsOpen, setSettingsOpen] = useState(false);

    useEffect(() => {
        backend.profile.all().then(setProfiles);
    }, []);

    const handleResize = (profile: Profile) => {
        backend.profile.apply(profile);
    }
    
    return (
        <div className="relative">
             <button className="btn btn-circle btn-outline btn-sm fixed top-4 left-4 z-[5]" onClick={() => setSettingsOpen(!settingsOpen)}>
                <SettingsIcon size={16}/>
            </button>
            <div className={`w-full h-screen settings-drawer ${settingsOpen ? 'sidebar-open' : ''}`}>
            <SettingsMenu
                settings={settings}
                updateSettings={updateSettings}
            />
            <div className="home-screen-grid grid h-screen grid-cols-[45%_55%] w-full bg-gradient-to-t from-[#660e99] to-[#941882]">
                <div className="logo-col h-full flex flex-col justify-center items-center pb-8 pr-8 pl-8 pt-8">
                    <img src="./resize-raccoon-logo.png" className="w-full max-w-full h-auto object-contain max-h-[50vh]"/>
                    <div className="text-container mt-4">
                        <span className="text-element text-4xl font-bold text-slate-200 tracking-wide font-noto">
                            ResizeRaccoon
                        </span>
                    </div>
                    <div className="only-home flex flex-col">
                        <div className="uppercase text-2xs font-bold tracking-wide">
                            <div className="version-info flex justify-center gap-1">
                                <div><span className="lowercase">v</span>{version}</div>
                                <div>•</div>
                                <a className="link" target="_blank" href="https://github.com/mistenkt/resize-raccoon">Homepage</a>
                            </div>
                        </div>
                        <div className="divider w-20 self-center mt-2 mb-2"/>
                        <div className="form-control flex w-full auto-resize-toggle">
                            <div className="tooltip tooltip-left self-center" data-tip="Process watching will automatically apply a profile when the application is">
                                <label className="label cursor-pointer justify-end gap-2 items-center p-0">
                                    <span className="label-text">Process watcher</span> 
                                    <input type="checkbox" className="toggle toggle-accent toggle-md" checked={settings.processWatcherEnabled} onChange={e => updateSettings({...settings, processWatcherEnabled: e.target.checked})
                                    }/>
                                </label>
                            </div>
                            
                        </div>
                        <button className="btn btn-outline mt-4" onClick={onNewProfile}>
                            <PlusCircle size={16}/>
                            New profile
                        </button>
                    </div>
                </div>
                <div className="flex flex-col h-full overflow-y-auto pr-8 pt-12 pb-12 w-full">
                    {profiles.map((p, i) => (
                        <ProfileListItem 
                            key={i}
                            profile={p}
                            onEdit={onEditProfile}
                            onResize={handleResize}
                        />
                    ))}
                </div>
            </div>
        </div>
        </div>
        
        
    )
}

export default HomeScreen;