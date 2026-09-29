# NanoMC Reworked 1.12.2

~ **Minecraft 1.12.2, portable and ready to run.** ~

A portable Minecraft 1.12.2 setup based on the original [NanoMC](https://github.com/skidsploiter/NanoMC) concept.

## [!] About

NanoMC Reworked is a **portable, no-install Minecraft 1.12.2 setup** designed to keep the entire game environment inside a single folder.

It includes:

- Minecraft 1.12.2
- Forge 14.23.5.2859
- Java 8 x64
- LWJGL and required libraries
- Portable game data
- Support for Forge mods
- Portable worlds, configurations and resource packs

The goal is simple:

**Extract → run → enter a username → play.**

No Minecraft launcher installation is required for the portable environment.

---

## [!] Important

NanoMC Reworked is **not an official Minecraft launcher** and is not affiliated with Mojang Studios or Microsoft.

This project is intended for educational, experimental and portable-use purposes.

You should own a legitimate copy of Minecraft and comply with Minecraft's applicable terms and licenses.

---

## [~] What is NanoMC Reworked?

NanoMC Reworked takes the original NanoMC idea and adapts it for **Minecraft 1.12.2 with Forge**.

Everything required to run the portable environment is kept inside the project folder.

The game uses a bundled **Java 8 x64 runtime**, meaning you do not need to install Java separately on the computer where you run it.

Your Minecraft data also stays inside `mcdata/`.

This means worlds, mods, configurations and resource packs can travel with the portable installation.

---

## [!] Features

- **Portable** → Run it from a normal folder, external drive or USB drive.
- **No Minecraft Launcher required** → The included launcher script starts Minecraft directly.
- **Built-in Java 8** → Java is included inside `mcdata/java/`.
- **Minecraft 1.12.2** → Uses Minecraft 1.12.2.
- **Forge support** → Forge 14.23.5.2859 is included.
- **Mod support** → Add Forge `.jar` mods directly to `mcdata/mods/`.
- **Portable worlds** → Worlds are stored inside `mcdata/saves/`.
- **Portable settings** → Game and mod configurations stay inside `mcdata/`.
- **Portable resource packs** → Resource packs are stored inside `mcdata/resourcepacks/`.
- **Simple launcher** → Run the `.bat` file and enter your username.

---

## [...] Requirements

### Operating system

- Windows
- Windows 10 or newer recommended

### Hardware

Minecraft 1.12.2 requirements depend heavily on the mods being used.

For a basic setup:

- 64-bit Windows
- At least 4 GB of system RAM
- At least 1 GB of free storage
- A GPU capable of running Minecraft 1.12.2

More RAM and a stronger GPU may be required for large modpacks.

---

## [~] Setup

1. Download or obtain the NanoMC Reworked folder.
2. Extract the folder anywhere you want.
3. You can place it on:
   - Your PC
   - An external drive
   - A USB flash drive
4. Open the NanoMC Reworked folder.
5. Run the launcher `.bat` file.
6. Enter the username you want to use.
7. Minecraft 1.12.2 will start.

No separate Java installation is required because Java 8 is included.

---

## [>] Playing

Every time you want to play, simply run the launcher `.bat` again.

The launcher will start Minecraft using the portable `mcdata/` directory.

Your username is requested when the launcher starts.

---

## [🌎] Worlds

Singleplayer worlds are stored in:

```text
mcdata/saves/
