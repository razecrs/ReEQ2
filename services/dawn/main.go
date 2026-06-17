// dawn is the web side of ReEQ2 — account login, server status, patch bits.
// the rust crates handle the actual game protocol; this just fronts the http.
package main

import (
	"log"
	"net/http"
	"os"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		_, _ = w.Write([]byte("ok"))
	})

	addr := os.Getenv("DAWN_ADDR")
	if addr == "" {
		addr = ":2424" // same port emagi's dawn admin sat on
	}

	log.Printf("dawn listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, mux))
}
