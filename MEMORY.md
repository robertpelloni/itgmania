# MEMORY.md

## Internal Observations
* Project uses CMake for build system.
* Linux dependencies: `libxinerama-dev`, `libxtst-dev`, `libxrandr-dev`, `libgtk-3-dev`, OpenGL, ALSA/PulseAudio, LibUSB.
* `fetch-extern-deps.sh` is used to fetch external dependencies to avoid recursive submodule fetching issues through proxies.
* Written primarily in C++ with extensive Lua integration for the frontend/theming.
