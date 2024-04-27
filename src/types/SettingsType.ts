interface Settings {
    processWatcherEnabled: boolean;
    pollRate: number;
    checkForUpdates: boolean;
    launchOnStart: boolean;
    hasPromptedForLaunchOnStart: boolean;
    startMinimized: boolean;
    closeToTray: boolean;
}

export default Settings;