import socket


def tcp_client(host="127.0.0.1", port=5000):
    # Create a TCP/IP socket
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    try:
        # Connect to server
        client_socket.connect((host, port))
        print(f"Connected to {host}:{port}")

        # Send some data
        for i in range(3):
            message = f"{i}: Hello, server!"
            client_socket.sendall(message.encode("utf-8"))
            print(f"Sent: {message}")

            # Receive response
            response = client_socket.recv(1024)
            print(f"Received: {response.decode('utf-8')}")

    except Exception as e:
        print(f"Error: {e}")

    finally:
        client_socket.close()
        print("Connection closed.")


if __name__ == "__main__":
    tcp_client(port=9123)
