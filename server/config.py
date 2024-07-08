from dotenv import load_dotenv
import psycopg2

import os

load_dotenv()


class Config:
    SQLALCHEMY_DATABASE_URI = os.getenv("DATABASE_URL")
    CONNECTION = psycopg2.connect(SQLALCHEMY_DATABASE_URI)
