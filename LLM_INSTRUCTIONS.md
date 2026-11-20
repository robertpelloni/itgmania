# LLM_INSTRUCTIONS.md

## General Directives
*   **Version single source of truth:** `VERSION.md`. Every build/task MUST increment this version number, explicitly update `CHANGELOG.md`, and reference the version bump in the associated git commit message.
*   **Documentation:** Always maintain, revise, and update a comprehensive suite of documentation files including `VISION.md`, `MEMORY.md`, `DEPLOY.md`, `CHANGELOG.md`, `ROADMAP.md`, `TODO.md`, `IDEAS.md`, and `HANDOFF.md`. All other AI instruction files must point here.
*   **Coding style:** Always comment code in profound detail (explaining what it does, why it is there, side effects, optimizations, and alternatives), but leave completely self-explanatory code bare.
*   **Autonomy:** Execute recommendations sequentially, maintaining autonomy. Proceed to the next feature automatically.

## Workflow
1.  **Initialization:** Read memory tools, repo structure, rules, and history. Infer architecture.
2.  **Repo Sync:** Sync upstream, merge branches, catch up to main, and update submodules recursively.
3.  **Roadmap Extraction:** Analyze codebase for gaps and populate `ROADMAP.md` and `TODO.md`.
4.  **Documentation:** Keep `VISION.md`, `MEMORY.md`, `DEPLOY.md`, `IDEAS.md`, and `CHANGELOG.md` updated.
5.  **Implementation:** Ensure UI representation for backend features, comment deeply. Update manuals.
6.  **Handoff/Termination:** Summarize findings, document in `HANDOFF.md`, sync server.
