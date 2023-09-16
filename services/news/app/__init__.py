from app.server import server
from os import environ

def main():
    print("Running server")
    port = int(environ.get("PORT", 4444))

    server.run(debug=True, port=port)
