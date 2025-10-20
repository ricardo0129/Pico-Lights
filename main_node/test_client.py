import socket


def tcp_client(host="127.0.0.1", port=5000):
    # Create a TCP/IP socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        print(f"Connected to server at {host}:{port}")
        bytes_to_send = bytes([1])
        s.sendall(bytes_to_send)
        print(f"Sent: {bytes_to_send}")



if __name__ == "__main__":
    tcp_client(host='192.168.4.92', port=4243)
