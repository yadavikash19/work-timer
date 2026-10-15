# Work Timer 2.0: one real app for Windows, Mac (and Linux), with auto-update

Decided 2026-10-15 by the owner: "I want a real app ... it should be auto-update." Replaces the PowerShell app (work-tracker/) and the friend's own Mac port.

## Stack
- **Tauri 2**: Rust core (tracking, files, local server, sync) + the existing dashboard.html as the UI. Small (about 10 MB download, light on RAM).
- **Builds on GitHub Actions** (Windows + macOS runners), not on the owner's laptop (8 GB RAM, under 1 GB free: a local Rust build would freeze it).
- **Updates**: Tauri updater, signed with our own free key; files and `latest.json` hosted on the existing Cloudflare site. The app asks "Update now / Remind me later (2 h)", same as 1.4.2.
- **Same data**: reads the same day files and settings.json, so history moves over.

## Milestones (estimates)
1. Skeleton: repo, CI builds a Windows .exe and a Mac .dmg, tray icon, a window, auto-update working end to end.
2. Tracker core (Windows first): idle time, window in front, browser titles, work / not work, YouTube words, day files, pauses taken back.
3. The box: 3 themes, drag, right-click menu, the edge half-circle (style C liquid, morph animation), nudges, sound and shake, the update question.
4. Dashboard: the same dashboard.html served by the app, every /api route ported.
5. Leaderboard sync + new group features: admin / mod, invite-only + approvals, kick, today's changes under the table, animated emoji names, Windows / Apple / Linux logos.
6. Mac: idle, app + window titles (Screen Recording permission), sound meter, login item, testing with the Mac friend.
7. The rest: study words + sound rule, topics, focus note, AI review (copy mode; claude mode on the owner's laptop), memes, phone time, Claude chat names.
8. Switch-over: a last PowerShell update (1.4.x) that offers to install 2.0, website download page for both systems.

## Open questions for the owner
- GitHub repo: private (free build minutes are limited, Mac minutes count 10x) or public (unlimited free builds, code visible)?
- Apple developer account ($99/year) to skip the Mac "Open anyway" step? Default: no.
