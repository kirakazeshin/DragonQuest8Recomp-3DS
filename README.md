# DragonQuest8Recomp-3DS
Pc port recomp of the 3DS version of Dragon Quest 8 based on Europe version

# Dragon Quest VIII – 3DS EUR Native PC Recompilation

> **Early Release / Work in Progress**

This project is an experimental native PC recompilation of the **European Nintendo 3DS version of Dragon Quest VIII: Journey of the Cursed King**.

## Important Note

I want to make something clear from the beginning:

**I do not know how to program.**

I started this project out of curiosity and used tools, documentation, existing open-source projects, and assistance available to me to see how far I could get.

This project would simply not exist without the work done by the developers of the recompilation tools and frameworks it relies on.

This is my first attempt at something like this, so please keep that in mind when reporting issues or looking through the project.

---

# First Release

This is the **first public release**, and it intentionally contains only the bare minimum required to get the game running as a native PC recompilation.

At this stage, the goal is not to present a finished "PC Remaster" of Dragon Quest VIII.

The goal of this release is to establish a working foundation that can be improved over time.

Current features include:

- Native Windows executable generated from the European 3DS version.
- Recompiled game code rather than running the entire game through traditional 3DS CPU emulation.
- Vulkan hardware rendering through Zakuro.
- Higher internal rendering resolution.
- PC/controller input support through the current Zakuro implementation.
- 16:9 display support at the presentation level.
- Experimental single-screen layout.
- The original 3DS bottom screen can be displayed as an overlay instead of permanently occupying a separate second-screen area.
- Original game content and gameplay remain largely untouched.

The game currently boots correctly, can be controlled, starts a new game, and reaches normal gameplay and battles.

This is still an **early and experimental release**, so bugs, performance problems, graphical issues, crashes, or unsupported situations should be expected.

---

# What Makes This Different From Normal 3DS Emulation?

This project is **not intended to simply package a 3DS emulator with Dragon Quest VIII**.

With traditional emulation, an emulator reproduces the Nintendo 3DS hardware environment and executes the original game inside that emulated system.

This project instead uses **static recompilation**.

The original ARM game code is analyzed and translated into code that can be compiled for the PC platform. The resulting game executable then runs with the runtime and hardware abstractions provided by the recompilation framework.

In simplified terms:

**Traditional emulation:**

`Dragon Quest VIII → Emulated Nintendo 3DS → PC`

**This project:**

`Dragon Quest VIII code → Static recompilation → Native PC executable + runtime`

This approach also gives the project much more interesting long-term possibilities.

Instead of being permanently constrained by the original dual-screen 3DS presentation, parts of the game can eventually be adapted specifically for PC.

The long-term goal is therefore not simply:

> "Play the 3DS version on PC."

It is closer to:

> "Turn the 3DS version into a proper PC version."

More I work on it, more I realize I can't play for full test, so if you wanna help with the tests you're welcome: https://discord.gg/Qk8wvCvPU7

---

# Current PC Interface Work

One of the first modifications made to the project is an experimental **single-screen layout**.

The original Nintendo 3DS game was designed around two physical screens. On PC, permanently displaying both screens makes the game still feel like it is being played through a handheld emulator.

The current prototype allows the main 3D screen to remain the primary display while the bottom screen is rendered as a smaller overlay.

This is only a temporary first step.

The planned interface redesign is much more ambitious:

- The exploration map should normally remain hidden and appear only when requested by the player.
- Battle HP/MP and party information should be integrated directly into the main battle screen.
- Battle commands and dialogue boxes should eventually be redesigned for a clean single-screen PC interface.
- The final goal is to remove the feeling of playing a dual-screen Nintendo 3DS game while preserving all necessary gameplay information.

None of this should change how Dragon Quest VIII fundamentally plays.

The objective is to adapt its presentation to PC.

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/6dff3ef5-e8ca-4fa6-b654-2b8a1943f125" />
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/52da2462-2dba-4812-a851-5fd141f43eb1" />
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/6f5bce1e-b9c7-4236-87a3-4f1a4a59eaee" />
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/9b72b6cb-ebd5-4637-8ff4-5151afcf0e80" />



---

# European Version

This project currently targets the **European release** of:

**Dragon Quest VIII: Journey of the Cursed King – Nintendo 3DS**

Title ID:

`000400000018F200`

Other regional versions are currently **not the focus of this project**.

---

# Game Files / Legal Notice

This project does **not** provide Dragon Quest VIII, a Nintendo 3DS ROM, or copyrighted game assets.

You must provide your own legally obtained copy of the supported European version of the game.

The objective of this repository is to provide the code and modifications necessary for the recompilation project, not to distribute Square Enix or Nintendo copyrighted content.

Dragon Quest, Dragon Quest VIII and related assets and trademarks belong to their respective owners.

This is an unofficial fan project and is not affiliated with or endorsed by Square Enix or Nintendo.

---

# Special Thanks

## 3dsrecomp

A huge thank you to the creator(s) and contributors of **3dsrecomp**.

The static recompilation work provided by 3dsrecomp is the foundation that made this experiment possible.

Without their work on analyzing and recompiling Nintendo 3DS software, I would never have been able to get a project like this running.

## Zakuro

Another huge thank you to the creator(s) and contributors of **Zakuro**.

Zakuro provides the runtime and much of the infrastructure that allows the recompiled game to actually operate on PC, including graphics, input and other hardware abstractions.

The PC-specific modifications in this project are being built on top of that work.

---

# Future Goals

This first release was only a starting point.

Some of the things I would eventually like to investigate include:

- A complete single-screen PC interface. (almost done)
- Toggleable exploration map. (done)
- Redesigned battle HUD. (done)
- Better integration of battle commands and dialogue into the main screen. (done)
- Improved PC presentation. 
- Higher-quality rendering. (don't know if possible)
- Performance improvements. 
- Investigation of true 60 FPS gameplay.
- Additional quality-of-life improvements.
- Better handling of content originally designed specifically around the two Nintendo 3DS screens. (almost done)

There is **no guarantee that all of these features will be implemented**.

For now, the priority is keeping the recompilation functional and gradually improving it without breaking the original game.

---

# Final Words

This project started with a very simple question:

**"Can I turn the 3DS version of Dragon Quest VIII into something that feels like a real PC version?"**

I had no programming knowledge when I started.

Thanks to the tools and open-source work created by much more knowledgeable developers, I managed to get the European 3DS version running as a recompiled Windows application and started experimenting with PC-specific modifications.

This first release is therefore not the finished product.

It is the **starting point**.

Unfortunately, as I'm limited to the development using AI, I need to wait one month if I use all of my tools,
wich is the case with this first release 

Thank you to everyone who made the tools behind this possible.
