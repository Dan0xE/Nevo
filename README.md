# Nevo

Nevo is a simple tool that extracts snapshots from Minecraft


>**Warning**
> This tool only works with Minecraft versions 1.18.1 and below! (but i'm optimistic that we will find away to extend that to newer versions)

---

## How can i use this?

- Download the [Legacy Minecraft Launcher](https://launcher.mojang.com/download/MinecraftInstaller.msi) and select a Minecraft Version equal or below 1.18.1

<br >

>![image](https://user-images.githubusercontent.com/23116945/223898999-65c2f767-6e89-461a-88fe-a246ddaddd28.png)

- Then Launch the game, if the game window pops up launch the tool, specify a username and select <br >
the path you want the snapshot to be copied to and then press "Launch Game"

<br >

>![image](https://user-images.githubusercontent.com/23116945/223899246-f3bfcecb-9aee-4cfd-b141-c609a97c89e0.png)

<br >

>![image](https://user-images.githubusercontent.com/23116945/223899582-2af4c2f2-ff9c-4d29-9da3-ed38421a85f0.png) 

- A command window with all the arguments should open and a second instance of the should open

- Now you can open the game world to LAN and join via the second instance!

<br >
 
>![image](https://user-images.githubusercontent.com/23116945/223900218-f9f3033f-a0a1-4a98-b175-5f34d52d1368.png)

---

>**Warning**
>This only works if the Snapshot User joins the User with the Original Copy, the game will kick you if you do it the other way arround.

<br >

>![image](https://user-images.githubusercontent.com/23116945/223900587-61c687bc-eb09-4160-a4f9-7abd9c4ac438.png)

---

>**Warning**
>Currently you need to set the Powershell Script Execution Policy to unrestricted, otherwise the executable will not be able to load the powershell script (i am working on a fix for that by either using wmi directly or wrapping the command in something like "powershell -ExecutionPolicy Bypass -File p.ps1".

<br >

>![image](https://www.stanleyulili.com/assets/images/posts/2019-08-25-running-scripts-is-disabled-on-this-system-powershell/powershell-error.jpg)

---

## How does the tool work?
We read the arguments that the Minecraft process spawns with, then we collect these arguments and change <br >
certain parameters (such as the path and the username, the user can specify those in the UI). <br >
Then we write these arguments back to a .bat file and copy the Minecraft Snapshot to the specified location <br >
and launch the copied snapshot with the changed arguments.

---

## What is this for?
You cannot launch two Instances of Minecraft on one PC and join a LAN world without getting the Error "Name already taken".

![image](https://user-images.githubusercontent.com/23116945/223901125-39e28c9f-7043-4a9b-b96d-04339b388722.png)


This Tool and changing the previous described parameters enables LAN multiplayer on one PC.

---

## Can i do this manually?
Sure, just follow the steps describes in [this video](https://www.youtube.com/watch?v=UNpvtNHUbCE)

---

>If you encounter any issues, don't hesitate to head over to the issue tab an open an issue about what you encountered!
