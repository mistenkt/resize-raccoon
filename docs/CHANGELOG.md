# Changelog

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
