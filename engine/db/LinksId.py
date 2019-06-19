from sqlalchemy import Column, Integer, String
from engine.db import Base


class Pricipal(Base):
    __tablename__ = 'streamer'

    id = Column(Integer, primary_key=True)
    name = Column(String)
    room_url = Column(String)
    remaining_time = Column(Integer)
