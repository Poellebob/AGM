# Project Overview: AGM - A Generic Mod-manager

AGM is a cross-platform mod manager, currently in early development, aiming to provide CLI, TUI, and GUI interfaces for managing game modifications.

## Project Structure

The project is a Rust workspace composed of several crates:

-   `agm-core`: Contains the core logic, data structures, and foundational utilities for mod management. All shared logic that is not specific to a user interface (CLI, TUI, GUI) should reside here to ensure reusability and avoid duplication. Crucially, `agm-core` must *not* have dependencies on any other crate within the AGM workspace. It should only depend on external crates and the Rust standard library. This ensures its role as the foundational, independent core logic. It is architected to be completely headless, containing no direct user I/O (like printing to the console or reading from standard input). All user interaction is delegated to the frontend crates (`cli`, `tui`, `gui`) through traits like `InstallReporter`.
-   `cli`: Provides the command-line interface for interacting with AGM.
-   `tui`: Intended for a terminal user interface.
-   `gui`: Intended for a graphical user interface.
-   `agm-url-handler`: Likely responsible for registering and handling custom URL schemes (e.g., `nxm://`).

No logic goes in cli, tui or gui unless it has someting to do with user input,
and nothing to do with to do with mod management so no logic gets written twice.

## Core Concepts

### Profiles

*   YAML files defining a game's mod structure, including paths and special "mod directories" (`Moddir`) where mods can be placed. The `path` field in the `game` section should contain the *entire* path to the game's installation directory.
*   The `agm-core/src/profile.rs` defines the `Profile` struct which reflects this YAML structure.

### Presets

*   YAML files intended to define collections of mods for specific games, including download URLs and target file placement.
*   **Discrepancy:** The `agm-core/src/preset.rs`'s `Preset` and `Mod` structs are *simpler* than described in the `README.md`. The current Rust structs lack the `url` field for mods and the nested `files` structure, which are present in the `README.md`'s example YAML.

### Configuration

*   Managed by `agm-core/src/config.rs`.
*   Stores global settings such as `profiles` (list of profile names), `presets` (list of preset names), and `nexus_api_key`.
*   Uses `serde_yaml` for serialization to and from `config.yaml` located in the user's config directory (e.g., `~/.config/AGM`).
*   Ensures necessary data and config directories exist (e.g., `~/.local/share/AGM`, `~/.config/AGM`).

### Nexus Mods Integration

*   The project plans to integrate with Nexus Mods for downloading.
*   `agm-core/src/nexus.rs` likely contains the logic for this.
*   The CLI includes functionality to set the `nexus_api_key`.
*   A URL handler (`run_url_handler` in `cli/src/lib.rs`) is present, capable of parsing `nxm://` URLs and attempting to fetch download links via the Nexus API.

### Inter-Process Communication (IPC)

*   `agm-core/src/ipc.rs` and related code in `cli/src/lib.rs` (`run_url_handler`) suggest a design where frontends (CLI, TUI, GUI) can communicate with a background process or daemon, possibly for URL handling or other long-running tasks. A socket path (`agm.sock`) is defined for this.

### Mod Installation

*   The core logic for mod installation resides in `agm-core/src/install.rs`.
*   The `install_mods` function handles unpacking zip files, generating sidecar `ModSpec` YAML files, and guessing file placements based on profile MIME types.
*   **Symlinking for File Management:** A key aspect of AGM's file management strategy is the use of symlinks. Instead of copying mod files, AGM creates symbolic links from the game's mod directories (determined by the `point`s in the `ModSpec`) to the actual mod files stored in a central location (`~/data-dir/storage/<game_name>/<mod_name>/`). This minimizes file duplication, saves disk space, and allows for near-instantaneous mod activation and deactivation.
*   UI interaction is abstracted through the `InstallReporter` trait, which is implemented by each frontend (e.g., `CliInstallReporter` in `cli/src/lib.rs`) to handle user prompts.

## Current Implementation Status

*   **Overall:** The project has a solid architectural foundation with a clear separation between core logic and frontends.
*   **`agm-core`:**
    *   Provides foundational data structures for `Profile`, `Preset`, and `ModSpec`.
    *   Handles core logic for mod installation, including file extraction, `ModSpec` generation, and automatic placement guessing.
    *   Handles basic configuration loading, saving, and directory management.
    *   Includes modules for Nexus API interaction and IPC.
    *   Is completely free of user I/O operations, with errors propagated and frontend interaction handled via traits.
*   **`cli`:**
    *   Uses `clap` for command-line argument parsing.
    *   Implements the `InstallReporter` trait for interactive installation via the command line.
    *   Handles all user interaction, such as prompting for input and displaying information.
    *   Most other commands (`profile`, `preset`) are still basic stubs.
*   **`tui` & `gui`:** These interfaces are present but not yet implemented. They are expected to implement the `InstallReporter` trait for their respective UIs.

In summary, AGM has a clear vision and a well-defined architecture with modular components. The installation process has been significantly developed, with a good separation of concerns. The next steps would be to fully implement the `profile` and `preset` management, and to build out the TUI and GUI frontends.
