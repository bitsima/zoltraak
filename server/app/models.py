from app import db


class CommandQueue(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    implant_uuid = db.Column(db.String, index=True)
    command = db.Column(db.String)
