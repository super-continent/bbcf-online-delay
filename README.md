# Blazblue: Centralfiction Online Input Delay Mod
This mod should be placed next to `BBCF.exe` or chain loaded from BBCF Improvement Mod.
It will create a new file `BBCF_ONLINE_FRAME_DELAY.txt` in the game folder, the default value is 1, but you can set it to whatever you like, any numbers in the 0-5 range probably work best.

To change the input delay after configuring it, a game restart is currently required (I wanted to add a UI slider but that requires hooking D3D9Ex stuff, so hudhook doesn't work with BBCF).

# Note:
Currently the mod seems to sometimes cause meter UI issues for a player who is both the creator of an online lobby and assigned player 1 in the game, I will be working to resolve this if possible, but for now it should work for anyone using the mod in all other cases
