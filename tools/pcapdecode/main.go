// pcapdecode is the RE harness — point it at a capture and it dumps the
// eqstream frames so i can check the rust protocol crate against real bytes.
// stub for now; wiring up pcapng parsing next.
package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	in := flag.String("in", "", "path to a .pcapng capture")
	flag.Parse()

	if *in == "" {
		fmt.Fprintln(os.Stderr, "usage: pcapdecode -in <file.pcapng>")
		os.Exit(2)
	}

	fmt.Printf("TODO: decode %s and print the eqstream frames\n", *in)
}
