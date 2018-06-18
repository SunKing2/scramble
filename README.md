# scramble

Scramble Word Game

Scramble was a word game hosted by Delphi in the 1990s.  Then it was hosted at scramblelovers.com for several years, with source code re-written in java. This is Scramble written in Rust.  

Scramble uses a simple chat server on port 21, accessible with telnet or any nicer client software.  It is hosted at scramblelovers.com

It is like Boggle, users around the world play a round that lasts 90 seconds.  They are shown a 4 X 4 grid of letters, and they make as many words as they can by entering them in their client software (telnet, for example).  Then the game shows who won that round.

Stats are available with chat commands, and any user can start a game similarly.  The server controls the timing, calculating etc.  This code is mostly the server. 

=======================
 user SunKing2 has enterd (4 players now)
 user cleetus as entered (5 players now)
Cleetus > hi peeps
Cleetus > rd
*** Get ready for a round of Scramble (90 Seconds) ***
C H L E
A T N I
O S U Q
R U S T

Round has ended... 
Scores: 
SunKing2   .... [score stuff ellided]

visit the live game at
http://scramblelovers.com

