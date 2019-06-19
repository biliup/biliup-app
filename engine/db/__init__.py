from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker, scoped_session
from engine import USER, PASSWORD, SOURCE, CATALOG

MSSQL_CONNECT_STR = 'mysql+pymysql://%s:%s@%s/%s?charset=utf8' % (USER, PASSWORD, SOURCE, CATALOG)

# echo=True 会显示每条执行的 SQL 语句
# pool_size 表示连接池数量，默认是5
# pool_recycle 单位秒，自动回收连接的秒数,
# 这对 MySQL 是必须的，默认 情况下 MySQL 会自动移除闲置 8 小时或者以上的连接。
# 需要注意地是如果使用 MySQL 的话， Flask-SQLAlchemy 会自动地设置这个值为 2 小时。


engine = create_engine(MSSQL_CONNECT_STR, echo=False, pool_size=10)

# 可以用此对象对数据库直接进行操作，最好是在http某个基类的__init__函数中赋值，如self.db = db_session，这样每次收到post或get请求都会自动重新创建连接

# 然后使用self.db操作，避免多线程使用同一个连接出现问题，即每个任务调用一次db_session
DBSession = scoped_session(sessionmaker(bind=engine))

# DBSession = sessionmaker(bind=engine)
Base = declarative_base()
