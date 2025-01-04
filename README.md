# Blazblue: Centralfiction Online Input Delay Mod
This mod should be placed next to `BBCF.exe` or chain loaded from BBCF Improvement Mod.
It will create a new file `BBCF_ONLINE_FRAME_DELAY.txt` in the game folder, the default value is 1, but you can set it to whatever you like, any numbers in the 1-4 range probably work best.

To change the input delay after configuring it, a game restart is currently required (I wanted to add a UI slider but that requires hooking D3D9Ex stuff, so hudhook doesn't work with BBCF).

# Note:
If both players have their delay value set to 0, player 1 will sometimes experience meter UI issues, for that reason I recommend a delay of 1 as the lowest value
