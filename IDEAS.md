# IDEAS.md

## Brainstorming & Feature Expansion
* Investigate WebAssembly port for browser-based play.
* Implement direct multiplayer matching without centralized server dependencies.
* Modernize rendering pipeline (Vulkan/Metal support).
* AI-driven chart generation or real-time difficulty adjustment.

## Future Refactoring Ideas
* **Asset Loading Optimization:** Use lazy-loading or prefetching queues for audio and video textures using a background thread pool instead of blocking.
* **State Management:** Fully decouple GAMESTATE into isolated component stores (e.g. `ScoreStore`, `PlayerStateStore`, `MusicStore`) to enforce robust threading architectures without mutex locks.
* **Lua Type Safety:** Implement or bridge a strictly typed interface from Lua to C++, possibly generating TypeScript/Luau headers.
