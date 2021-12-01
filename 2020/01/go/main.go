// AOC 2020 day 1
// & having some fun with goroutines!

package main

import (
  "bufio"
  "fmt"
  "os"
  "strconv"
  "strings"
  "sync"
)

const target = 2020

type empty struct{}

type res2 struct {
  i0, i1 int
}

type res3 struct {
  i0, i1, i2 int
}

func main() {
  ns, err := readNumbers("input")
  if err != nil {
    panic(err)
  }

  for _, r := range findTwo(ns) {
    n0, n1 := ns[r.i0], ns[r.i1]
    sum, prod := n0+n1, n0*n1
    fmt.Println(fmt.Sprintf("indices (%d,%d): %d + %d = %d, %d * %d = %d",
      r.i0, r.i1, n0, n1, sum, n0, n1, prod))
  }

  for _, r := range findThree(5, ns) {
    n0, n1, n2 := ns[r.i0], ns[r.i1], ns[r.i2]
    sum, prod := n0+n1+n2, n0*n1*n2
    fmt.Println(fmt.Sprintf("indices (%d,%d,%d): %d + %d + %d = %d, %d * %d * %d = %d",
      r.i0, r.i1, r.i2, n0, n1, n2, sum, n0, n1, n2, prod))
  }
}

func findTwo(ns []int) []res2 {
  var results []res2

  for i0 := 0; i0 < len(ns); i0++ {
    for i1 := i0; i1 < len(ns); i1++ {
      if ns[i0]+ns[i1] == target {
        results = append(results, res2{i0, i1})
      }
    }
  }

  return results
}

func findThree(gs int, ns []int) []res3 {
  var results []res3

  inputs := make(chan int, gs)
  result := make(chan res3, 0)

  doneSem := make(chan empty, 1)

  wgSem := make(chan empty, 1)
  var wg sync.WaitGroup

  for gid := 0; gid < gs; gid++ {
    wg.Add(1)

    go func() {
      defer wg.Done()
      for {
        i0, more := <-inputs
        if !more {
          break
        }
        for i1 := i0; i1 < len(ns); i1++ {
          for i2 := i1; i2 < len(ns); i2++ {
            if ns[i0]+ns[i1]+ns[i2] == target {
              result <- res3{i0, i1, i2}
            }
          }
        }
      }
    }()
  }

  go func() {
    defer func() { doneSem <- empty{} }()
    loop:
    for {
      select {
      case <-wgSem:
        break loop
      case r := <-result:
        results = append(results, r)
      }
    }
  }()

  for i := 0; i < len(ns); i++ {
    inputs <- i
  }
  close(inputs)

  wg.Wait()
  wgSem <- empty{}
  <-doneSem

  return results
}

func readNumbers(path string) ([]int, error) {
  fd, err := os.Open(path)
  if err != nil {
    return nil, err
  }
  defer fd.Close()
  scanner := bufio.NewScanner(fd)
  var ns []int
  for scanner.Scan() {
    line := strings.TrimSpace(scanner.Text())
    if len(line) == 0 {
      continue
    }
    n, err := strconv.Atoi(line)
    if err != nil {
      return nil, err
    }
    ns = append(ns, n)
  }
  return ns, nil
}
