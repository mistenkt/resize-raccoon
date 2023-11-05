import { useEffect, useMemo, useState } from "react";
import Process from "../types/ProcessType";
import { Profile } from "../types/ProfileTypes";
import { v4 as uuidv4 } from 'uuid';
import backend from "../utils/backend";
import { presets } from "../constants";
import { Info, RefreshCw } from "react-feather";

interface Props {
    onCancel: () => void;
    onSaved: () => void;
    profile?: Profile;
}

const ProfileEditor = ({ onCancel, onSaved, profile }: Props) => {
    const [processes, setProcesses] = useState<Process[]>([]);
    const [name, setName] = useState<string>(profile?.name || '');
    const [selectedProcess, setSelectedProcess] = useState<Process>();
    const [processName] = useState<string>(profile?.processName || '');
    const [windowWidth, setWindowWidth] = useState<string>(String(profile?.windowWidth) || '');
    const [windowHeight, setWindowHeight] = useState<string>(String(profile?.windowHeight) || '');
    const [windowPosX, setWindowPosX] = useState<string>(String(profile?.windowPosX) || '');
    const [windowPosY, setWindowPosY] = useState<string>(String(profile?.windowPosY) || '');
    const [autoResize, setAutoResize] = useState<boolean>(profile?.auto || false);
    const [delay, setDelay] = useState<string>(String(profile?.delay) || '')
    const [loadingProcesses, setLoadingProcesses] = useState<boolean>(false);
    const uuid = useMemo(() => {
        if(profile) {
            return profile.uuid;
        }

        return uuidv4();
    }, [profile]);


    useEffect(() => {
        backend.process.running().then(setProcesses);
    }, []);

    const getUpdatedProfile = (): Profile => ({
        uuid,
        name,
        processName: processName || selectedProcess?.name || '',
        windowWidth: Number(windowWidth),
        windowHeight: Number(windowHeight),
        windowPosX: Number(windowPosX),
        windowPosY: Number(windowPosY),
        delay: Number(delay),
        auto: autoResize,
    });

    useEffect(() => {
        if(selectedProcess && !name) {
            setName(selectedProcess.name.split('.exe')[0]);
        }
    }, [selectedProcess, name]);

    const onTest = () => {
        const testProfile = getUpdatedProfile();
        backend.profile.apply(testProfile);
    }

    const testEnabled = useMemo(() => {
        return windowWidth !== '' && windowHeight !== '' && (processName || selectedProcess);
    }, [selectedProcess, processName, windowWidth, windowHeight]);

    const canSave = useMemo(() => {
        return (name && (processName || selectedProcess) && windowWidth && windowHeight);
    }, [selectedProcess, processName, windowWidth, windowHeight, name])

    const handleSave = () => {
        const updatedProfile = getUpdatedProfile();
        let endpoint = profile ? backend.profile.update : backend.profile.add;
        endpoint(updatedProfile)
            .then(() => onSaved());
    }

    const handleDelete = () => {
        if(!profile) return;
        backend.profile.delete(profile)
            .then(() => onSaved());
    }

    const handlePresetSelection = (presetName: string) => {
        const preset = presets.find(p => p.name === presetName);
        if(!preset) return;

        setWindowWidth(String(preset.windowWidth));
        setWindowHeight(String(preset.windowHeight));
        setWindowPosX(String(preset.windowPosX));
        setWindowPosY(String(preset.windowPosY));
    }

    const handleReloadProcesses = () => {
        if(loadingProcesses) return;
        setLoadingProcesses(true);
        Promise.all([
            backend.process.running().then(setProcesses),
            new Promise(resolve => setTimeout(resolve, 500)) // Its too fast, so we add a delay so its more clear that it has loaded and not just a flicker
        ]).finally(() => {
            setLoadingProcesses(false);
        });
    }

    return (
        <div className="flex flex-col h-screen bg-gradient-to-t from-[#660e99] to-[#941882]">
            <div className="pb-4 pt-8 pl-8 flex flex-row items-center gap-4">
                <img src="./resize-raccoon-logo.png" className="w-fullw-[80px] h-[80px] object-contain"/>
                <span className="text-element text-4xl font-bold text-slate-200 tracking-wide font-noto mt-2">
                    ResizeRaccoon
                </span>
            </div>
            <div className="flex-grow overflow-hidden w-full h-[100vh] grid grid-rows-[1fr_auto] pr-8 pl-8 pb-8">
                <div className="grid grid-cols-2 gap-8 mb-8 ">
                    <div>
                        <div className="form-control w-full">
                            <label htmlFor="process" className="label pb-1 justify-start gap-2">
                                <span className="text-2xs uppercase font-semibold">Process</span>
                                <div className="tooltip before:w-[300px] before:-left-[10px] before:translate-x-0" data-tip="The application you want to create a resize profile for. If you launched the program after opening this screen you should click the refresh button.">
                                    <Info size=".8em"/>
                                </div>
                                
                            </label>
                            <div className="flex gap-2">
                                <select
                                    id="process"
                                    className="select w-full"
                                    onChange={e => {
                                        setSelectedProcess(processes.find(p => p.pid === parseInt(e.target.value)));
                                    }}
                                    value={selectedProcess?.pid}
                                    defaultValue="default"
                                >
                                    {processName && !selectedProcess ? (
                                        <option value="default">{processName}</option>
                                    ) : (
                                        <option value="default">Select a process</option>
                                    )}

                                    {processes.map((p) => (
                                        <option key={p.pid} value={p.pid}>{p.name}</option>
                                    ))}
                                </select>
                                <button className="btn btn-outline" onClick={handleReloadProcesses}>
                                    {loadingProcesses ? (
                                        <span className="loading loading-spinner w-[16px]"></span>
                                    ) : (
                                        <RefreshCw size={16} />
                                    )}
                                    
                                </button>
                            </div>
                            
                        </div>
                        <div className="form-control w-full mb-4">
                            <label htmlFor="name" className="label pb-1 justify-start gap-2">
                                <span className="text-2xs uppercase font-semibold">Preset name</span>
                                <div className="tooltip before:w-[300px] before:-left-[10px] before:translate-x-0" data-tip="Whatever you want to call your profile">
                                    <Info size=".8em"/>
                                </div>
                                
                            </label>
                            <input type="text" id="name" className="input w-full" value={name} onChange={e => setName(e.target.value)} />
                        </div>
                    </div>
                    <div>
                        <div className="form-control w-full">
                            <label htmlFor="preset" className="label pb-1 justify-start gap-2">
                                <span className="text-2xs uppercase font-semibold">Preset</span>
                                <div className="tooltip before:w-[300px]" data-tip="Pre calculated values for some triple monitor setups">
                                    <Info size=".8em"/>
                                </div>
                                
                            </label>
                            <select
                                id="preset"
                                className="select w-full"
                                onChange={e => handlePresetSelection(e.target.value)}
                                defaultValue="default"
                            >
                                <option value="default">Copy values from preset</option>
                                {presets.map((p) => (
                                    <option key={p.name} value={p.name}>{p.name}</option>
                                ))}
                            </select>
                        </div>
                        <div className="grid grid-cols-2 gap-4">
                            <div className="form-control w-full">
                                <label htmlFor="window_width" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Window Width</span>
                                    <div className="tooltip before:w-[300px]" data-tip="Probably self-explanatory, the intended width of the window. For triple monitor setups this is 3x a single screens horizontal pixels.">
                                        <Info size=".8em"/>
                                    </div>
                                    
                                </label>
                                <input type="number" id="window_width" className="input w-full" value={windowWidth} onChange={e => setWindowWidth(e.target.value)} />
                            </div>
                            <div className="form-control w-full">
                                <label htmlFor="window_height" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Window Height</span>
                                    <div className="tooltip before:w-[300px] before:left-auto before:right-0 before:translate-x-0" data-tip="Probably self-explanatory, the indended height of the window. For triple monitor setups this is usually just the horizontal pixels of a single screen">
                                        <Info size=".8em"/>
                                    </div>
                                </label>
                                <input type="number" id="window_height" className="input w-full" value={windowHeight} onChange={e => setWindowHeight(e.target.value)} />
                            </div>
                        </div>
                        <div className="grid grid-cols-2 gap-4 mb-4">
                            <div className="form-control w-full">
                                <label htmlFor="window_pos_x" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Window Position X</span>
                                    <div className="tooltip before:w-[300px]" data-tip="The horizontal position of the window. Depends on what is defined as your main monitor, and the layout of your screens. Usually a negative matching the horizontal pixels of one screen">
                                        <Info size=".8em"/>
                                    </div>
                                    
                                </label>
                                <input type="number" id="window_pos_x" className="input w-full" value={windowPosX} onChange={e => setWindowPosX(e.target.value)}/>
                            </div>
                            <div className="form-control w-full">
                                <label htmlFor="window_pos_y" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Window Position Y</span>
                                    <div className="tooltip before:w-[300px] before:left-auto before:right-0 before:translate-x-0" data-tip="The vertical position for the window (usually 0)">
                                        <Info size=".8em"/>
                                    </div>
                                    
                                </label>
                                <input type="number" id="window_pos_y" className="input w-full" value={windowPosY} onChange={e => setWindowPosY(e.target.value)}/>
                            </div>
                        </div>

                        <div className="grid grid-cols-2 gap-4 mb-4">
                            <div className="form-control w-full">
                                <label htmlFor="auto_resize" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Auto resize</span>
                                    <div className="tooltip before:w-[300px]" data-tip="Allow this profile to be automatically applied when the program is launched (requires global process watching)">
                                        <Info size=".8em"/>
                                    </div>
                                    
                                </label>
                                <div className="h-[48px] flex items-center">
                                    <input id="auto_resize" type="checkbox" className="toggle toggle-accent toggle-lg" checked={autoResize} onChange={e => setAutoResize(e.target.checked)
                                        }/>
                                </div>           
                            
                            </div>
                            <div className="form-control w-full">
                                <label htmlFor="auto_delay" className="label pb-1 justify-start gap-2">
                                    <span className="text-2xs uppercase font-semibold">Auto resize delay (ms)</span>
                                    <div className="tooltip before:w-[300px] before:left-auto before:right-0 before:translate-x-0" data-tip="The application you want to create a resize profile for">
                                        <Info size=".8em"/>
                                    </div>
                                    
                                </label>
                                <input type="number" id="auto_delay" className="input w-full" value={delay} onChange={e => setDelay(e.target.value)}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div className="pb-8 pr-8 pl-8">
                <div className="divider pt-0 mt-0"></div>
                <div className="grid grid-cols-3 gap-4">
                    <div className="text-left">
                        <button 
                            className="btn btn-outline btn-info" 
                            onClick={onTest}
                            disabled={!testEnabled}
                        >Test</button>
                    </div>
                    <div className="text-center">
                        {!!profile && (
                            <button className="btn btn-outline btn-secondary mr-4" onClick={handleDelete}>Delete</button>
                        )}
                        <button className="btn btn-outline" onClick={onCancel}>Cancel</button>
                    </div>
                    <div className="text-right">
                        <button className="btn" onClick={handleSave} disabled={!canSave}>Save</button>
                    </div>
                    

                </div>
            </div>
            
        </div>
        
    )
}

export default ProfileEditor;