from sqlalchemy.dialects.postgresql import JSONB

from app import db


class CommandQueue(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    implant_uuid = db.Column(db.String, index=True)
    commands = db.Column(JSONB)

    def __repr__(self):
        return f"<CommandQueue(id={self.id}, implant_uuid={self.implant_uuid}, commands={self.commands})>"
