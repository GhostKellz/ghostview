# 👻 GhostView

![GhostView Preview](assets/preview.png)

[![Build](https://img.shields.io/github/actions/workflow/status/ghostkellz/ghostview/ci.yml?label=CI\&logo=github\&style=flat-square)](https://github.com/ghostkellz/ghostview/actions)
[![License: MIT](https://img.shields.io/github/license/ghostkellz/ghostview?color=green\&style=flat-square)](LICENSE)
[![Stars](https://img.shields.io/github/stars/ghostkellz/ghostview?style=social)](https://github.com/ghostkellz/ghostview)
[![Rust](https://img.shields.io/badge/Built_with-Rust-informational?logo=rust\&style=flat-square)](https://www.rust-lang.org)
[![GPU Ready](https://img.shields.io/badge/EGL%20%7C%20Wayland-ready-blue?style=flat-square\&logo=nvidia)](https://wiki.archlinux.org/title/Wayland)

> ✨ A sleek, modular GUI for discovering and managing packages across the Arch Linux ecosystem — built with Rust and `egui`.

---

## 🎯 What is GhostView?

**GhostView** is a beautiful, lightweight desktop interface for exploring the best Linux packages — powered by [`ghostbrew`](https://github.com/ghostkellz/ghostbrew) under the hood.

Easily browse, search, and manage:

* 📦 **Pacman** (system and official repos)
* 🔹 **AUR** and **Chaotic-AUR**
* 📦 **Flatpak** (Flathub & custom remotes)
* 🧩 KDE / GNOME Apps
* 🔧 GitHub-sourced developer tools
* 🐧 (Coming soon) Snap, AppImage, and Debian packages

---

## 💡 Key Features

| Feature                                          | Status                    |
| ------------------------------------------------ | ------------------------- |
| 🔍 Unified package search (Pacman, AUR, Flatpak) | ✅ Pacman/AUR/Flatpak done |
| 📄 Full package details & clipboard copy         | ✅ Done                    |
| 🌐 Sidebar filtering by source/type              | ✅ Done                    |
| 🎮 Keyboard navigation                           | ✅ Done                    |
| 🌙 Async-ready design & status bar               | ✅ Initial implementation  |
| 🖼️ egui-based modern GUI                        | ✅ Stable                  |
| 🧠 LLM-enhanced discovery & tagging              | ⚠️ Experimental           |

---

## ⚙️ Tech Stack

* 🦀 **Rust** for performance, stability, and portability
* 🖼️ **egui** + `eframe` for native, fast, GPU-accelerated GUI
* 🐧 Built for Linux/Wayland first (no Electron, no web stack)

---

## 📦 Goals

* Create a smooth, discoverable alternative to tools like Discover, Octopi, or Pamac
* Provide real package insights across multiple backends (Pacman + AUR + Flatpak)
* Allow fast install/update/removal with full user control
* Stay minimal and performant

---

## 🔮 Future Vision

* 📎 Link to `ghostbrew-repo` instances
* 📱 Extend to mobile & web dashboard (optional)
* 🔐 Auth + remote sync (multi-device package dashboards)
* 🧠 LLM-based suggestions or app tagging
* ✅ Flatpak permissions & sandbox info display

---

## 🧙‍♂️ Get Involved

* Submit feature ideas or bugs via GitHub Issues
* Contribute Rust or Go code to `ghostview` and `ghostbrew`
* Help expand support for Snap, AppImage, and more
* Spread the word if you find it useful!

---

© 2025 [CK Technology](https://cktechx.com) / [GhostKellz](https://ghostkellz.sh) – Licensed under [MIT](LICENSE)
