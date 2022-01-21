# DD1338 Week 14

We hope you have had a great time with your holidays, and your code has been blessed! Now its time to get on coding (for school) again. Yay! 

## The Internet is for... Fun!

This weeks task is to work with sockets! To do this, you will have to select a programming language of your choice and figure out how sockets work in that language. Most languages, like Java, have built in objects for sockets that are really easy to work with. Rust offers networking functionality via its standard library.

To get you started, you can read [this guide](https://realpython.com/python-sockets/) on _*psudokod_ Python _*host *host_ sockets. The guide also shares its source code [here](https://github.com/realpython/materials/tree/master/python-sockets-tutorial). 
*// Tobbe*

To **properly** get started, see the Rust book's [multithreaded server example project](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). 
_// Viola_

### Prepare your assignment

1. Create a repository named `<KTH_ID>-sockets` (Haskellers: You already have your repos in `inda-21`).
2. Start working on your prefered assignment.

I (Tobias) have provided some of my old Java code under `./java-examples` a server and client program. The client sends two numbers to the server that adds them together and send the result back to the client. Your job will be to make something more advanced but the example code should set you off if you want to use Java.

I (Viola) touched up a very simple Rust server and a client binary under `./rust-example`. Check it out!

## Assignments

The Rustacean have the single choise of numero いち. 

The Haskeller can, on the other hand, instead choose to implement networking within their chistmas game project.

### いち. Make a chat client and server!

The main point of this week is to make a server and client that can communicate with each other using sockets. For this assignment, we want your server to support multiple clients, meaning that two (or more) clients can connect and communicate with each other through the server.

_Note_: If your solution is built on top of an existing example, make sure that your applications are distinct by adding fun new features. For example direct messaging, client names/colours, file sharing...

_Intervension_: Who wants to only chat? You're allowed to develop a simple LAN game instead.

### に. Improve your game(s)!

If you want to you could add some form of multiplayer functionality to your games from last week. This could be to allow player vs. player (PvP), or you could do that as well as adding an ingame chat while playing multiplayer. This option allows you to work alone or continue working with your partner from `game-task-12/13`