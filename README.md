# curly-train
A simple game written in rust that allows to play pong between two computers.
Be sure to use cargo run from main directory, and use linux since termion doesn't work on windows.
## Build
Clone the repo, and use cargo run, that's it.
## How to play?
Find yourself a friend, then run the app. One person has to be a "host", and the other one should connect to host socket. The rest of game is organized in P2P structure, there isn't any central server, players both check each positions and validate it.
To move your paddle use "W" and "S". If the screen appears to be broken, try to zoom in or zoom out in terminal, it's broken because board_size.x in conny.sp has too big board_size_x value. You can also play on 1 computer using two apps : )

## Conny file
File contains information about connection that will be made with other user, conny.sp must be the same to establish connection.
The values in conny are respectivly, proto_ver, ball speed in x direction, ball speed in y direction, board_size_x, board_size_y, paddle speed and draw trail flag which should be 1 or 0 
