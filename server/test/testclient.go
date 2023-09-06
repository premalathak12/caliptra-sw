// client.go

package main

import (
	"fmt"
	"net"
	"os"
)

func main() {
	socketPath := "/tmp/socket_example.sock"

	// Connect to the server
	conn, err := net.Dial("unix", socketPath)
	if err != nil {
		fmt.Println("Error connecting to server:", err)
		os.Exit(1)
	}
	defer conn.Close()

	// Send Power On command to start the server
	//conn.Write([]byte("poweron\n"))

	// Send Command 1
	conn.Write([]byte("command1\n"))

	// Read and print Command 1 response from server
	response := make([]byte, 1024)
	n, err := conn.Read(response)
	if err != nil {
		fmt.Println("Error reading from server:", err)
		return
	}
	fmt.Println("Server response:", string(response[:n]))

	// Send Command 2
	conn.Write([]byte("command2\n"))

	// Read and print Command 2 response from server
	n, err = conn.Read(response)
	if err != nil {
		fmt.Println("Error reading from server:", err)
		return
	}
	fmt.Println("Server response:", string(response[:n]))

	// Send Power Off command to stop the server
	conn.Write([]byte("poweroff\n"))
}

