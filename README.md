# RustyChess

## Motivation

This project kicked off during my exchange program in Ireland, organized by my school. During the first two weeks, I didn't have any assigned projects, so I started working on my own. A friend suggested we build a chess bot. He already had a Python chess engine, but we couldn't agree on what kind of bot to make. He wanted something fancy like a neural network, and I just wanted to mess around with algorithms. So, in the end, I decided to create my own chess engine from scratch using Rust.

I also threw in a MySQL database because I knew we'd eventually be working with APIs and databases, so I figured, why not get some practice while I'm at it?

I didn't finish the project before the official assignment came in, but I still want to see it through.

## Updates

I try to document my journey here every time I work on the project (this is like a blog). This should motivate me and it will probably be interesting seeing how I felt or how I progressed.

[15.FEB.2025] \
**Motivation:** 9/10 \
**What I did:** I uploaded the project to a GitHub repo and wrote the motivation and Decisions chapter. \
**Notes:** I'm at "[The Clockwork Door](https://www.clockworkdoor.ie/)", which is a cool spot in Dublin. I'm excited to keep tinkering with this, because it's challenging and just plain fun. Plus, I'm learning a few fundamentals along the way. \

[16.FEB.2025] \
**Motivation:** 8/10 \
**What I did:** I worked on the Decisions chapter. \
**Notes:** I wanted to get some work done but couldn't find the motivation to start. When I finally sat down with my laptop, though, I just kept writing and didn't stop.

[17.FEB.2025] \
**Motivation:** 8/10 \
**What I did:** I worked on the migration scripts. \
**Notes:** I saw that my current database setup didn't make any sense so I rewrote it.

[18.FEB.2025] \
**Motivation:** 4/10 \
**What I did:** I worked on the MySQL connection pool. \
**Notes:** I was sick, so I was not really produktiv. I looked at some documentation and tried implementing a connection pool. I can't test it because for some reason does my environment variable that I habe in .env not exist.

[19.FEB.2025] \
**Motivation:** 5/10 \
**What I did:** I worked on fixing the env "bug" and removed every bug. \
**Notes:** It is hard to work on it, because I now need to work on things in the office and I don't find the motivation to do something/start at home. At least, I do something every day.

## Decisions

I had a course in school about "distributed systems", which was really hard, because it was a group project and as always with such projects I did way too much work in comparison to my teammates. But I learned something really important and that are design decisions. I never actively thought about them. I used to just work on things and go with the first idea I had, which worked 99% of the time. But on this project I actively thought about solutions, which are surprisingly many. From how to save the board state to how to communicate moves.

| Problem                                                                | Solution                                                                                                                                                                                                                                                                                                                                                 | Example                                                                                                                                                                  |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Save pieces in a way that is easily extendable (add new custom pieces) | Save pieces and their movement options in database so I don't have to code new pieces and can just write them into the db.                                                                                                                                                                                                                               | Movement rule looks like this "(∣x2−x1∣\*∣y2−y1∣)=2", this would be for a knight. I’m not sure if all pieces can be represented this way, but I'll figure it out if not. |
| Representing a board                                                   | I saw a video from "the Cherno" ([found it](https://www.youtube.com/watch?v=NeHjMNBsVfs&t=786s) ) even though I never watched it to the end I still remembered that the chess engine he reviewed saved every piece in a u64, which makes sense, but I was not a fan of it. In the end, I decided to use a two-dimensional array to store all the pieces. | Still not sure which the better solution is.                                                                                                                             |
| Saving board state.                                                    | I will probably use a binary file format, because it is the most memory efficient solution I know of and if I want to save a lot of games that is needed. The only problem is that it is not human readable, but that should not be a problem.                                                                                                           | There is a rust library that does that so I will probably use that and maybe if I finish the main project, I might try building my own.                                  |
