import { useEffect, useState } from "react";
import "./App.css";
import ProfileEditor from "./screens/ProfileEditor";
import { Profile } from "./types/ProfileTypes";
import backend from "./utils/backend";
import { getVersion } from '@tauri-apps/api/app';
import HomeScreen from "./screens/HomeScreen";
import Settings from "./types/SettingsType";

enum Screen {
    HOME = 'home',
    PROFILE_EDITOR = 'profile_editor',
}

function App() {
    const [screen, setScreen] = useState<Screen>(Screen.HOME);
    const [editProfile, setEditProfile] = useState<Profile>();
    const [initialized, setInitialzied] = useState<boolean>(false);
    const [version, setVersion] = useState<string>('');
    const [settings, setSettings] = useState<Settings>();

    useEffect(() => {
        (async () => {
            const [settings, version] = await Promise.all([
                backend.settings.all(),
                getVersion(),
            ]);

            setSettings(settings);
            setVersion(version);
            setTimeout(() => setInitialzied(true), 100);
        })();
    }, []);

    useEffect(() => {
        if(!initialized || !settings) return;
        backend.settings.update(settings);
    }, [settings, initialized])

    const handleNewProfile = () => {
        setEditProfile(undefined);
        setScreen(Screen.PROFILE_EDITOR);
    }

    const handleEditProfile = (profile: Profile) => {
        setEditProfile(profile);
        setScreen(Screen.PROFILE_EDITOR);
    }

    const handleSaveProfile = () => {
        setEditProfile(undefined);
        setScreen(Screen.HOME);
    }

    const getScreen = () => {
        if(!settings) return;
        switch (screen) {
            case Screen.HOME:
                return (
                    <HomeScreen
                        version={version}
                        onNewProfile={handleNewProfile}
                        onEditProfile={handleEditProfile}
                        settings={settings}
                        updateSettings={setSettings}
                    />
                )
            
            case Screen.PROFILE_EDITOR:
                return (
                    <ProfileEditor
                        profile={editProfile}
                        onCancel={() => setScreen(Screen.HOME)}
                        onSaved={handleSaveProfile}
                    />
                )
            default:
                return <div>Unknown screen</div>
        }
    }

    return getScreen();
}

export default App;
