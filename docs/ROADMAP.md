# Resize Raccoon Roadmap

Quick overview over features i plan to implement
|
## Current Status
- Current version: `v1.2.0`
- [View Changelog](./CHANGELOG.md)

### Very soon™
- **Global resize delay**: Adding a global resize delay setting that will affect all processes.
- **Bug Fixes**: List any known bugs that will be addressed.

## Soon™
- **Process Watcher v1 (non-beta)**: Sort out the issues currently faced where some games are making adjustments to their windows during boot preventing a profile from being appliedp properly. 
    - Detect when a program is "ready" for receiving new size/position params. 
    - Re-enabling a profile if its overridden by the process itself at any point.
    - See if we can find a better solution for detecting when a process is started than polling.
- **Testing**: Add tests
- **More options in profiles**: Introduce more of the "advanced" options that SRWE offers to provide a more "complete" sollution. Open for suggestions on features to prioritize here.

## Contributing
If you want to contribute on the project, or discuss features feel free to open a PR or ISSUE. Or contact me directly. If this at any point gets some traction and someone wants to join in on the development i'll setup a discord or something for communication.

## Feedback
Please report any bugs or feature requests through an issue. 

- [Submit Feedback](https://github.com/mistenkt/resize-raccoon/issues)
