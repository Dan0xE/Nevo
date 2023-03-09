# Nevo

Nevo is a simple tool that extracts snapshots from minecraft


>**Warning**
> This only works with Minecraft versions 1.18.1 and below

---

## How does this work?
We read the arguments that the Minecraft process spawns with, then we collect these arguments and change <br >
certain parameters (such as the path and the username, the user can specify those in the UI). <br >
Then we write these arguments back to a .bat file and copy the Minecraft Snapshot to the specified location <br >
and launch the copied snapshot with the changed arguments. 

---

## What is this for?
You cannot launch two Instances of Minecraft on one PC and join a LAN world without getting the Error "Name already taken".

<br>
This Tool and changing the previous described parameters enables LAN multiplayer on one PC
<br>
<br>

>**Warning**
>This only works if the Snapshot User joins the User with the Original Copy, the game will kick you if you do it the other way arround

>**Warning**
>Currently you need to set the Powershell Script Execution Policy to unrestricted, otherwise the executable will not be able to load the powershell script (i am working on a fix for that by either using wmi directly or wrapping the command in something like "powershell -ExecutionPolicy Bypass -File p.ps1"
