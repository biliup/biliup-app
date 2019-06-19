import yaml
from common.event import Event
from common.reload import autoreload
from common.timer import Timer


# def getmany(_links_id):
#     _urls = []
#     urlstatus = {}
#     for k, v in _links_id.items():
#         _urls += v
#         for url in v:
#             urlstatus[url] = 0
#     return _urls, urlstatus





def main(event_manager):
    # 初始化定时器
    timer_ = Timer(func=event_manager.send_event, args=(Event(CHECK),), interval=20)

    # 模块更新自动重启
    autoreload(event_manager, timer_, interval=15)

    event_manager.start()
    timer_.start()


CHECK = 'check'
TO_MODIFY = 'to_modify'
DOWNLOAD_UPLOAD = 'download_upload'
BE_MODIFIED = 'be_modified'
UPLOAD = 'upload'


with open(r'config.yaml', encoding='utf-8') as stream:
    config = yaml.load(stream, Loader=yaml.FullLoader)
    # links_id = config['links_id']
    user_name = config['user_name']
    pass_word = config['pass_word']
    chromedrive_path = config['chromedrive_path']

    USER = config['db_usr']
    PASSWORD = config['db_psw']
    SOURCE = config['source']
    CATALOG = config['catalog']


# urls, url_status = getmany(links_id)
__all__ = [
    'downloader', 'upload', 'plugins', 'main',
    # 'urls', 'url_status',
    'CHECK', 'BE_MODIFIED', 'DOWNLOAD_UPLOAD', 'TO_MODIFY', 'UPLOAD',
    "USER", "PASSWORD", "SOURCE", "CATALOG"
    ]
