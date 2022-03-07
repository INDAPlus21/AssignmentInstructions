### Deadline:
This work should be completed before the exercise on **Friday 16th April**.

### Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-20/course-instructions#assignments).

### Homework
Study the following course literature:

- Read the following from the [Fundamentals of Concurrent Programming](http://yourbasic.org/golang/concurrent-programming/)
  - [Mutual exclusion](http://yourbasic.org/golang/mutex-explained/)
  - [Efficient parallel computation](http://yourbasic.org/golang/efficient-parallel-computation/)

### Task 1 - Matching Behaviour

Take a look at the program [matching.go](src/matching.go). Explain what happens and why it happens if you make the following changes. Try first to reason about it, and then test your hypothesis by changing and running the program.

  * What happens if you remove the `go-command` from the `Seek` call in the `main` function?
  * What happens if you switch the declaration `wg := new(sync.WaitGroup`) to `var wg sync.WaitGroup` and the parameter `wg *sync.WaitGroup` to `wg sync.WaitGroup`?
  * What happens if you remove the buffer on the channel match?
  * What happens if you remove the default-case from the case-statement in the `main` function?

Hint: Think about the order of the instructions and what happens with arrays of different lengths.

### Task 2 - Fractal Images

The file [julia.go](src/julia.go) contains a program that creates images and writes them to file. The program is pretty slow. Your assignment is to divide the computations so that they run in parallel on all available CPUs. Use the ideas from the example in the [efficient parallel computation](http://yourbasic.org/golang/efficient-parallel-computation/) section of the course literature.

You can also make changes to the program, such as using different functions and other colourings.

How many CPUs does you program use? How much faster is your parallel version?

> **Assistant's note:** In more recent versions of Golang (since 1.5), the runtime will default to use as many operating system threads as it is allowed. To see differences in behaviour, refer to the [GOMAXPROCS](https://golang.org/pkg/runtime/#GOMAXPROCS) function and vary the value.

### Task 3 - MapReduce

In the final task, you will be applying the MapReduce model for improving a word frequency program.

> **Assistant's note:** We actually struggled to find a nice introductory explanation beyond our own example below, as the full model has several more layers that we are skipping. If you are curious, check out the original Google research article and see how you get on! https://ai.google/research/pubs/pub62

A word frequency analysis of a document will return a summary of the word counts for all unique words in the document. Whilst this can be solved efficiently using a map data structure in a sequential program, the performance can be improved by parallelising the program.

> **Assistant's note:** Typically we ignore case when counting the frequency so 'Hello', 'HELLO' and 'hello' are all counted as 'hello', so remember to convert all strings to lower case.

By splitting the document into sub-documents and conducting a partial count in parallel (_Map_ task), we can arrive at the solution by combining all the partial results into a final result (_Reduce_ task).

Read the code in [singleworker/words.go](src/singleworker/words.go) and complete the missing parts:

- Implementation for the `WordCount` function
- Reading a text file into a string in the `main` function
- Check that the unittest passes
- Log the runtime performance in the table below

Once you are satisfied with the singleworker, move into [mapreduce/words.go](src/mapreduce/words.go) and parallelise the program in order to improve the performance.

- Update the `WordCount` function with the Map and Reduce tasks, using goroutines to parallelise and a channel to gather partial results
- Check that the unittest passes
- Find the optimal amount of gorountines before you encounter diminishing returns in performance improvements
- Log the runtime performance in the table below


|Variant       | Runtime (ms) |
| ------------ | ------------:|
| singleworker |          xxx |
| mapreduce    |          yyy |

And with that, you are on your way to Google-scale problems ;-)


---

Please commit any written answers or diagrams to the "docs" folder as a PDF (or Markdown) document, and commit any code developed to the "src" folder of your KTH Github repo. Remember to push to KTH Github.
