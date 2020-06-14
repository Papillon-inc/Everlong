#!/bin/bash

cp /Users/kunosouichirou/Documents/GitHub/TheColourAndTheShape/monkey_wrench/internal/buildinfo/buildinfo.go.in /Users/kunosouichirou/Documents/GitHub/TheColourAndTheShape/monkey_wrench/internal/buildinfo/buildinfo.go
go generate /Users/kunosouichirou/Documents/GitHub/TheColourAndTheShape/monkey_wrench/internal/api
go run *.go ${@:1}