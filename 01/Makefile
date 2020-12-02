GO_SRC := go
GO_MAIN := main.go
GO_EXEC := 01-go
HS_SRC := haskell
HS_MAIN := main.hs
HS_EXEC := 01-hs

all: go haskell

go:
	go build -o $(GO_EXEC) $(GO_SRC)/$(GO_MAIN)

haskell:
	ghc -o $(HS_EXEC) $(HS_SRC)/$(HS_MAIN)

clean: clean-go clean-haskell

clean-go:
	rm $(GO_EXEC)

clean-haskell:
	rm $(HS_EXEC) $(HS_SRC)/*.hi $(HS_SRC)/*.o

bench-go:
	go test -run=Bench -bench=. ./go

.PHONY: all go haskell clean clean-go clean-haskell bench-go
