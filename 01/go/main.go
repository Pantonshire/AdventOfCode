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

func main() {
  ns, err := readNumbers("input")
  if err != nil {
    panic(err)
  }
  findTwo(10, ns)
  fmt.Println("Done two")
  findThree(10, ns)
  fmt.Println("Done three")
}

func findTwo(gs int, ns []int) {
  type res2 struct {
    i0, i1 int
  }
  inputs := make(chan int, gs)
  result := make(chan res2, 0)
  doneSem := make(chan empty, 1)
  wgSem := make(chan empty, 1)
  var wg sync.WaitGroup
  for gid := 0; gid < gs; gid++ {
    wg.Add(1)
    go func() {
      for {
        i0, more := <-inputs
        if !more {
          wg.Done()
          break
        }
        for i1 := 0; i1 < len(ns); i1++ {
          if ns[i0]+ns[i1] == target {
            result <- res2{i0, i1}
          }
        }
      }
    }()
  }
  go func() {
    for {
      select {
      case <-wgSem:
        doneSem <- empty{}
        break
      case r := <-result:
        n0, n1 := ns[r.i0], ns[r.i1]
        sum, prod := n0+n1, n0*n1
        fmt.Println(fmt.Sprintf("indices (%d,%d): %d + %d = %d, %d * %d = %d",
          r.i0, r.i1, n0, n1, sum, n0, n1, prod))
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
}

func findThree(gs int, ns []int) {
  type res3 struct {
    i0, i1, i2 int
  }
  inputs := make(chan int, gs)
  result := make(chan res3, 0)
  doneSem := make(chan empty, 1)
  wgSem := make(chan empty, 1)
  var wg sync.WaitGroup
  for gid := 0; gid < gs; gid++ {
    wg.Add(1)
    go func() {
      for {
        i0, more := <-inputs
        if !more {
          wg.Done()
          break
        }
        for i1 := 0; i1 < len(ns); i1++ {
          for i2 := 0; i2 < len(ns); i2++ {
            if ns[i0]+ns[i1]+ns[i2] == target {
              result <- res3{i0, i1, i2}
            }
          }
        }
      }
    }()
  }
  go func() {
    for {
      select {
      case <-wgSem:
        doneSem <- empty{}
        break
      case r := <-result:
        n0, n1, n2 := ns[r.i0], ns[r.i1], ns[r.i2]
        sum, prod := n0+n1+n2, n0*n1*n2
        fmt.Println(fmt.Sprintf("indices (%d,%d,%d): %d + %d + %d = %d, %d * %d * %d = %d",
          r.i0, r.i1, r.i2, n0, n1, n2, sum, n0, n1, n2, prod))
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
