package api

import (
	"../hls"
	"net/http"
	"strconv"
)

func handleFrame(w http.ResponseWriter, r *http.Request) {
	t := r.URL.Query().Get("t")
	time := 30
	if tint, err := strconv.Atoi(t); err == nil {
		time = tint
	}
	file := getEntry(r).Path()
	hls.WriteFrame(file, time, w)
}