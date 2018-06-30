# scramble : Scramble Word Game

Try now in your browser: [http://alwayswantedtoplay.com]

Scramble was a word game hosted by Delphi in the 1990s.  Then it was hosted at scramblelovers.com for several years, with source code re-written in Java. 
This is Scramble written in Rust and Elm

---
####Start geeky developer stuff wanting to use this source code ----
* git clone https://github.com/SunKing2/scramble.git
* cd scramble
* cd rust_src
* cargo run
* # it'll tell ya what to do next to see some stuff.
* # end geeky developer stuff
---

Scramble uses a simple chat server on port 21, accessible with telnet or any nicer client software.  It is hosted at scramblelovers.com

Stats are available with chat commands, and any user can start a game similarly.  The server controls the timing, calculating etc.  This code is mostly the server. 

It is like Boggle, users around the world play a round that lasts 90 seconds.  They are shown a 4 X 4 grid of letters, and they make as many words as they can by entering them in their client software (telnet, for example).  Then the game shows who won that round.

This is Scramble written in Rust.  

```telnet
 user SunKing2 has enterd (4 players now)
 user cleetus as entered (5 players now)
Cleetus > hi peeps
Cleetus > rd
*** Get ready for a round of Scramble (90 Seconds) ***
C H L E
A T N I
O S U Q
R U S T

> [...elided stuff... I madly type in words, pressing Enter after each one...]

Round has ended... 
Scores: 
SunKing2   .... [score stuff elided]
```

The chat is typical, allowing talking before or during the game, private messaging, and blocking of individual users.

visit the live game at
http://alwayswantedtoplay.com

