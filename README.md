# Minecraft Snapshot Grabber

A simple tool that extracts snapshots from minecraft


>**Warning**
> This only works with Minecraft versions 1.18.1 and below

---

## How does this work?
Minecraft spawns a new Java.exe process with arguments. <br>
We collect these arguments and change certain parameters such as the path and the username. <br >
Those are specified by these User

Then we write these arguments back to a .bat file copy the Minecraft Snapshot to the specified location and launch it with the changed arguments

---

## What is this for?
You cannot launch two Instances of Minecraft on one PC and join a LAN world without getting the Error "Name already taken"

<br>
This enables to play LAN multiplayer on one PC
<br>
<br>

>**Warning**
>This only works if the Snapshot User joins the User with the Original Copy, it will kick you the other way arround

**Warning**
>Currently you need to set the Powershell Script Execution Policy to unrestricted, otherwise the executable will not be able to load the powershell script (i am working on a fix for that by either using wmi directly or wrapping the command in something like "powershell -ExecutionPolicy Bypass -File p.ps1"