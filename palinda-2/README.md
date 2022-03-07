### Deadline:
This work should be completed before the exercise on **Thursday 1st April**.

### Instructions
For instructions on how to do and submit the assignment, please see
the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-20/course-instructions#assignments).

### Homework
Study the following course literature:

- Read the following from the [Fundamentals of Concurrent Programming](http://yourbasic.org/golang/concurrent-programming/)
  - [Channels](http://yourbasic.org/golang/channels-explained/)
  - [Select](http://yourbasic.org/golang/select-explained/)
  - [Data races](http://yourbasic.org/golang/data-races-explained/)
  - [Detecting data races](http://yourbasic.org/golang/detect-data-races/)
  - [Deadlock](http://yourbasic.org/golang/detect-deadlock/)

### Task 1 - Debugging Concurrent Programs
In this task, you will be provided with two buggy programs. For each program,
you should:

1. Explain what is wrong with the code.
2. Fix the bug.
3. Explain why your solution fixes the bug.

#### Buggy program 1
[src/bug01.go](src/bug01.go)

#### Buggy program 2
[src/bug02.go](src/bug02.go)

> **Assistant's requirement:** You are not allowed to move or remove the
> time.Sleep(10 * time.Millisecond) statement in the Print function. A
> deterministic solution (e.g. using a channel or WaitGroup) is required and
> removing the sleep is only a quick fix.

### Task 2 - Many Senders; Many Receivers
The program [many2many.go](src/many2many.go) contains four
producers that together send 32 strings over a channel. At the
other end there are two consumers that receive the strings.
Describe what happens, and explain why it happens, if you make the
following changes in the program. Try first to reason your way
through, and then test your hypothesis by changing and running the
program.

* What happens if you switch the order of the statements
  `wgp.Wait()` and `close(ch)` in the end of the `main` function?
* What happens if you move the `close(ch)` from the `main` function
  and instead close the channel in the end of the function
  `Produce`?
* What happens if you remove the statement `close(ch)` completely?
* What happens if you increase the number of consumers from 2 to 4?
* Can you be sure that all strings are printed before the program
  stops?

Finally, modify the code by adding a new WaitGroup that waits for
all consumers to finish.

### Task 3 - Pythia, the Oracle of Delphi

The code in [oracle.go](src/oracle.go) contains the outline for a
program that will answer 'questions'.  Complete the `Oracle`
function.  You should not modify the `main` function or other
function signatures. Note that answers should not appear
immediately; instead there should be a delay or **pause for
thought**.  Also, the Oracle will still print **helpful
predictions** even if there are not any questions.  You may
structure your solution into multiple functions.

Your program should contain two channels: One channel for questions,
and one for answers and predictions.  In the `Oracle` function you
should start three indefinite go-routines.

* A go-routine that receives all questions, and for each incoming
  question, creates a separate go-routine that answers that question
* A go-routine that generates predictions
* A go-routine that receives all answers and predictions, and prints
  then to stdout

Whilst the `Oracle` function is the most important of the
assignment, you may also want to improve the answer-algorithm.

* The [strings](https://golang.org/pkg/strings/) and
  [regex](https://golang.org/pkg/regexp/) packages may be of some
  help
* The program can seem more human if the Oracle prints it answers
  one character at a time
* Take a look at the story of
  [ELIZA](https://en.wikipedia.org/wiki/ELIZA)

---

Please commit any written answers or diagrams to the "docs" folder
as a PDF (or Markdown) document, and commit any code developed to
the "src" folder of your KTH Github repo. Remember to push to KTH
Github before your exercise.
