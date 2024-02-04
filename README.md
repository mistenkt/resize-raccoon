<p align="center">
  <img src="./public/resize-raccoon-512x512.png" alt="Resize Raccoon Logo" width="200" height="200">
</p>

# Resize Raccoon

> A Windows window resize manager.

## Features

- **Automatic Window Resizing**: Set your preferred window dimensions and let Resize Raccoon do the rest
- **Accessible Profiles**: Create profiles for different applications and scenarios
- **Match profile to process once**: After defining a profile you don't need to match it to the process again, manually trigger the preset with a single click as long as the target program is running
- **Apply profiles from external applications/shortcuts**: Trigger your profiles from your stream deck or whatever

[![Thumbnail description](./public/home-screenshot-thumb.png)](./public/home-screenshot.png)
[![Thumbnail description](./public/profile-screenshot-thumb.png)](./public/profile-screenshot.png)

Check the [ROADMAP.md](./docs/ROADMAP.md) for upcoming features.

## Installing

To install Resize Raccoon, head over to the [Releases](https://github.com/mistenkt/resize-raccoon/releases) section and download the latest version for your operating system.

## Usage

If you prefer a visual guide, Dan Suzuki has a [Youtube video](https://www.youtube.com/watch?v=ei5UAPHBp7o) showing the app in action.

After installing Resize Raccoon, you can create custom profiles for your applications:

1. Open the Resize Raccoon interface.
2. Add a new profile by specifying the target application process, name, size and position and whatever else is there.
    1. If you are unsure about the values you can use some of the presets created for different triple monitor setups.
    2. You can test your profile before saving it to verify that it works. 
    3. Enable "Auto-Resize" to allow this profile to be automatically applied when your target process is launched.
    4. You can add a custom delay if we need to wait a little before resizing.
3. Once a profile is saved you will see it on the main screen, clicking the little window square on the profile will manually trigger the resize.
4. For the automatic resize to work, you need to enable process watching on the home screen.
5. Minimize to system tray and don't worry about having to resize your applications manually each you launch them. 
6. Polling interval can be ajusted in the sidebar -- it is set to 1000ms by default, in my testing it was very resource friendly so it should not be a problem.

### Triggering from Stream Deck

As long as the application is running you can trigger profiles from a .bat file or just from cmd directly using `echo apply-profile {profileName} > \\.\pipe\resize-raccoon`. If your profile name contains spaces please wrap it in quotes `echo apply-profile "my profile" > \\.\pipe\resize-raccoon`

For triggering profiles with Stream Deck you can either create a .bat file containting the command, or install the streamdeck-commandline plugin found here [mikepowell/streamdeck-commandline](https://github.com/mikepowell/streamdeck-commandline).

[StreamDeck icon](./public/resize-raccoon-streamdeck.png)

NB. There could be cases where the process you are trying to resize wont allow you unless you run `ResizeRaccoon` as admin.

## Running Locally

To run Resize Raccoon locally for development or personal use, follow these steps:

1. Clone the repository:
   ```sh
   git clone https://github.com/mistenkt/resize-raccoon.git```
2. Navigate to the project directory
    ```sh
    cd resize-raccoon
    ```
3. Install the necessary dependencies:
    ```sh
    cargo install tauri-cli
    ```
4. Run the application in development mode:
    ```sh
    cargo tauri dev
    ```

## Motivation and Inspiration

I hate Nvidia Surround and needed a way to resize borderless windows to span across 3 monitors when playing certain sim racing games.

Inspired by [Simple Runtime Window Editor (SRWE)](https://github.com/dtgDTGdtg/SRWE) which has a bunch more features, but didn't have a solution for automatically applying profiles to processes. You also had to manually select the process every time which was tedious. Great app though but it hasn't received updates in many years.

I also wanted to try coding with Rust.

## Why the name Raccoon?

It sounded cute and I'm sure if they had access to the Windows API they would be great at managing your windows.

## Tech Stack

- Rust
- Tauri
- Typescript
- React
- ChatGPT for some of the Rust/Tauri stuff
- Dall-E for the cute illustration

## Contributing

Feel free to open PRs or suggest features!
