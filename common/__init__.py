import logging.config
import os
from datetime import datetime, timezone, timedelta

log_file_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'configlog.ini')
logging.config.fileConfig(log_file_path)
logger = logging.getLogger('log01')


def time_now():
    utc_dt = datetime.utcnow().replace(tzinfo=timezone.utc)
    bj_dt = utc_dt.astimezone(timezone(timedelta(hours=8)))
    # now = bj_dt.strftime('%Y{0}%m{1}%d{2}').format(*'...')
    now = bj_dt.strftime('%Y{0}%m{1}%d').format(*'..')
    return now


def except_hook():
    import sys
    sys.excepthook = lambda t, v, tb: logger.error("Uncaught exception：", exc_info=(t, v, tb))
