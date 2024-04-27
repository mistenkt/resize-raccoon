# Changelog

## [1.7.0] - 2024-04-27
#### Added
- Close to tray setting. Enabled by default. No longer prompting when closing.
#### Fixed
- Forced dark mode to prevent style clashing for users with light-mode enabled.

## [1.6.0] - 2023-11-21
#### Added
- Borderless option (Fixes ACC)

## [1.5.0] - 2023-11-20
#### Added
- Start with windows settings option
- Start minimized settings option
#### Fixed
- Fixed bug where profile test button didnt stop loading after profile had been applied

## [1.4.0] - 2023-11-17
#### Added
- Massively improved performance of process watching. Like over 9000
- Added auto update feature. Checks on launch with dialog.


## [1.3.0] - 2023-11-17
#### Added
- After applying profiles, the system now keeps an eye on the process for a short while to make sure the size/position is not being changed agian by the process it self. If it detects a change between the applied profile and the window, it will reapply it self.
- If a process does not have an active window when applying the profile (like during load when the process hasnt had time to create its active window yet), we ill wait for 5 seconds and check to see if an active window has been created. There is a max of 2 retries for this. 
- As a result of the two fixes above automatic resizing can now be enabled without any delays


## [1.2.2] - 2023-11-17
#### Fixed
- Fixed problem where the wrong child window of a process was selected for resizing. (should fix issues with WRC and ACC not always resizing. Process watcher is still a bit buggy so recommend manually triggering for now, or setting a delay long enough where you know you have had time get passed the intro and have activated the main window)

## [1.2.1] - 2023-11-16
#### Fixed
- Bug with default profiles file not being created after refactorign some code

## [1.2.0] - 2023-11-12
#### Added
- Apply profiles from scripts or external programs using IPC.
    > As long as the application is running you can trigger profiles from a .bat file or just from cmd directly using `echo apply-profile {profileName} > \\.pipe\resize-raccoon`. If your profile name contains spaces please wrap it in quites `echo apply-profile "my profile" > \\.pipe\resize-raccoon`

## [1.1.0] - 2023-11-11
#### Added
- Error toasts
- Base for internationalization 
- Process poll rate setting

#### Fixed
- Bug where applying a profile for an application that has multiple processes might not always select the process that has a window.
- Position of some tooltips
- Bunch of other misc improvements, refactors.

## [1.0.1] - 2023-11-05
#### Fixed
- Profile view not resetting after editing
- Homepage link

## [1.0.0] - 2023-11-05
#### Added
- Everything
