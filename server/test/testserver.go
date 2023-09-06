// server.go

package main

import (
	"fmt"
	"net"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	// Create a Unix socket
	socketPath := "/tmp/socket_example.sock"
	l, err := net.Listen("unix", socketPath)
	if err != nil {
		fmt.Println("Error listening:", err)
		return
	}
	defer l.Close()

	fmt.Println("Server is listening on", socketPath)

	// Handle graceful shutdown
	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		sig := <-sigCh
		fmt.Printf("Received signal %v. Shutting down...\n", sig)
		os.Remove(socketPath)
		os.Exit(0)
	}()

	for {
		conn, err := l.Accept()
		if err != nil {
			fmt.Println("Error accepting connection:", err)
			continue
		}
		go handleRequest(conn)
	}
}

func handleRequest(conn net.Conn) {
	defer conn.Close()

	buf := make([]byte, 1024)
	for {
		n, err := conn.Read(buf)
		if err != nil {
			fmt.Println("Error reading from connection:", err)
			break
		}
		request := string(buf[:n])

		switch request {
		case "command1":
			// Handle Command 1
			fmt.Println("Received Command 1 from client.")
			conn.Write([]byte("Command 1 response from server.\n"))
		case "command2":
			// Handle Command 2
			fmt.Println("Received Command 2 from client.")
			conn.Write([]byte("Command 2 response from server.\n"))
		case "poweroff":
			// Handle Power Off
			fmt.Println("Received Power Off command from client. Shutting down...")
			os.Remove("/tmp/socket_example.sock")
			os.Exit(0)
		default:
			fmt.Println("Unknown command received:", request)
		}
	}
}

