from app.database import db
from app.database.models import CommandQueue

import logging

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
