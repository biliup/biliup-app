import multiprocessing
import time
import engine
import common
from engine import CHECK, BE_MODIFIED, DOWNLOAD_UPLOAD, TO_MODIFY, UPLOAD
from engine.db import DBSession
from engine.db.LinksId import Pricipal
from engine.downloader import download, Extractor
from engine.upload import Upload
from common import logger
from common.event import Event

# 初始化事件管理器

event_manager = common.event.EventManager()


@event_manager.register(DOWNLOAD_UPLOAD, block=True)
def process(name, url, mod):
    try:
        now = common.time_now()
        if mod == 'dl':
            p = multiprocessing.Process(target=download, args=(name, url))
            p.start()
            p.join()
            # download(name, url)
            Upload(name).start(url, now)
        elif mod == 'up':
            Upload(name).start(url, now)
        else:
            return url
    finally:
        event = Event(BE_MODIFIED)
        event.args = (url,)
        # return url
        return event


@event_manager.server()
class KernelFunc:
    def __init__(self):
        self.uploading = []
        self.batches = self.onebyone = None
        self.url_status = dict.fromkeys(list(map(
            lambda x: x.room_url,  DBSession().query(Pricipal).filter(Pricipal.remaining_time > 0).all())), 0)
        DBSession.remove()

    @event_manager.register(CHECK, block=True)
    def all_check(self):
        pricipals = DBSession().query(Pricipal).filter(Pricipal.remaining_time > 0).all()

        self.batches, self.onebyone = Extractor().sorted_checker(list(map(lambda x: x.room_url, pricipals)))

        live = []
        try:
            for batch in self.batches:
                res = batch.check()
                if res:
                    live.extend(res)

            for one in self.onebyone:
                for url in one.url_list:

                    if one('检测' + url, url).check_stream():
                        live.append(url)

                    if url != one.url_list[-1]:
                        logger.debug('歇息会')
                        time.sleep(15)

        except IOError:
            logger.exception("IOError")
        finally:
            DBSession.remove()

            event_u = Event(UPLOAD)
            event_u.args = (live,)

            event_t = Event(TO_MODIFY)
            event_t.args = (live,)
            return event_u, event_t

    @event_manager.register(engine.TO_MODIFY)
    def modify(self, live_m):
        live_d = {}
        if live_m:
            event = []
            for live in live_m:
                live_d[live] = 1
                if self.url_status.get(live) != 1:
                    name = DBSession().query(Pricipal).filter(Pricipal.room_url == live).one().name
                    logger.info(name + '刚刚开播，去下载')
                    event_d = Event(DOWNLOAD_UPLOAD)
                    event_d.args = (name, live, 'dl')
                    event.append(event_d)

            self.url_status = dict.fromkeys(list(map(
                lambda x: x.room_url, DBSession().query(Pricipal).filter(Pricipal.remaining_time > 0).all())), 0)

            self.url_status.update(live_d)
            # url_status = {**url_status_base, **live_d}
            DBSession.remove()
            return tuple(event)

    @event_manager.register(engine.UPLOAD)
    def free_upload(self, _urls):
        logger.debug(_urls)
        event = []
        pricipals = DBSession().query(Pricipal).all()
        DBSession.remove()
        for pricipal in pricipals:
            title = pricipal.name
            url = pricipal.room_url
            if url not in self.uploading and self.url_status.get(url) != 1 and Upload(title).filter_file():
                event_d = Event(DOWNLOAD_UPLOAD)
                event_d.args = (title, url, 'up')
                event.append(event_d)

                self.uploading.append(url)
        return tuple(event)

    @event_manager.register(engine.BE_MODIFIED)
    def revise(self, url):
        if url:
            # url_status = {**url_status, **{url: 0}}
            if url in self.uploading:
                self.uploading.remove(url)
            else:
                self.url_status.update({url: 0})
