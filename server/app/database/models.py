from sqlalchemy.dialects.postgresql import JSONB, BYTEA

from app import db


class CommandQueue(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    implant_uuid = db.Column(db.String, index=True)
    commands = db.Column(JSONB)

    def __repr__(self):
        return f"<CommandQueue(id={self.id}, implant_uuid={self.implant_uuid}, commands={self.commands})>"


class FileChunk(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    implant_uuid = db.Column(db.String, index=True)
    file_id = db.Column(db.String, index=True)
    chunk_index = db.Column(db.Integer, index=True)
    chunk_data = db.Column(BYTEA)

    def __repr__(self):
        return f"<FileChunk(id={self.id}, implant_uuid={self.implant_uuid}, file_id={self.file_id}, \
            chunk_index={self.chunk_index}, total_chunks={self.total_chunks})>"
