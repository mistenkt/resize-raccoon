const en = {
    "toast": {
        "buttons": {
            "dismiss": "Dismiss"
        }
    },
    "profile": {
        "process": {
            "title": "Process",
            "description": "The application you want to create a resize profile for. If you launched the program after opening this screen you should click the refresh button.",
            "select": "Select a process"
        },
        "profileName": {
            "title": "Profile name",
            "description": "The name of the profile you want to create. This will be used to identify the preset in the list of profiles.",
        },
        "preset": {
            "title": "Preset",
            "description": "Pre calculated values for some triple monitor setups.",
            "select": "Copy values from preset",
            "options": {
                "triple1080p": "Triple 1080p",
                "triple1440p": "Triple 1440p",
                "triple4k": "Triple 4k",
            }
        },
        "window" : {
            "width": {
                "title": "Window width",
                "description": "Probably self-explanatory, the intended width of the window. For triple monitor setups this is 3x a single screens horizontal pixels.", 
            },
            "height": {
                "title": "Window height",
                "description": "Probably self-explanatory, the intended height of the window. For triple monitor setups this is the height of a single screen.", 
            },
            "posY": {
                "title": "Window position y",
                "description": "The vertical position of the window on the screen. Usually 0.",
            },
            "posX": {
                "title": "Window position x",
                "description": "The horizontal position of the window on the screen. Depends on your setup. For triple monitors usually a negative value equal to the width of a single screen.",
            },
        },
        "autoResize": {
            "title": "Auto resize",
            "description": "Allow this profile to be automatically applied when the program is launched (requires global process watching).",
            "enabled": "Automatic",
            "disabled": "Manual",
        },
        "autoResizeDelay": {
            "title": "Auto resize delay (ms)",
            "description": "The delay in milliseconds between the program launching and the profile being applied.",
        },
        "buttons": {
            "test": "Test profile",
            "cancel": "Cancel",
            "save": "Save",
            "delete": "Delete",
        },
    },
    "home": {
        "homepage": "Homepage",
        "processWatcher": "Process watcher",
    },
    "settings": {
        "checkForUpdates": {
            "title": "Check for updates on launch",
        },
        "processPollRate": {
            "title": "Process poll rate (ms)",
            "description": "How often should we check for new applications being launched.",
        },
        "launchOnStart": {
            "title": "Start with windows"
        }
    },
    "errors": {
        "window_manager": {
            "process_not_found": "Process not found, make sure the application your trying to control is running.",
            "apply_failed": "Failed to apply profile, not sure why. Try again?",
            "access_deined": "Failed to apply profile, access denied. Try running Resize Raccoon as admin.",
            "invalid_pid": "Failed to apply profile, invalid process id. You'll never see this error. If you do, you win a prize.",
        },
        "profile": {
            "not_found": "Profile not found, restart the program to sync profiles.",
            "profile_path_error": "Couldnt locate the profile path, this is bad, but shouldnt happen. Idk, just dont delete the application data folder I guess.",
        },
        "settings": {
            "launch_on_start_error": "Unable to toggle launch on start",
            "unable_to_fetch_app_data": "Unable to fetch application meta data",
            "settings_path_error": "Could not locate the settings.json file",
        }
    }
}

export default en;