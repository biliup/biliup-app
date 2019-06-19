import requests
from engine.plugins import YDownload, BatchCheckBase
from common import logger

headers = {
    'client-id': 'jzkbprff40iqj646a697cyrvl0zt2m6'
}
VALID_URL_BASE = r'(?:https?://)?(?:(?:www|go|m)\.)?twitch\.tv/(?P<id>[0-9_a-zA-Z]+)'
API_ROOMS = 'https://api.twitch.tv/helix/streams'
_API_USER = 'https://api.twitch.tv/helix/users'


class Twitch(YDownload):
    def __init__(self, fname, url, suffix='mp4'):
        YDownload.__init__(self, fname, url, suffix=suffix)


class BatchCheck(BatchCheckBase):
    def __init__(self, urls):
        BatchCheckBase.__init__(self, pattern_id=VALID_URL_BASE, urls=urls)
        self.use_id = {}
        if self.usr_list:
            login = requests.get(_API_USER, headers=headers, params={'login': self.usr_list}, timeout=5)
            login.close()
        else:
            logger.debug('无twitch主播')
            return
        try:
            for pair in login.json()['data']:
                self.use_id[pair['id']] = pair['login']
        except KeyError:
            logger.info(login.json())
            return

    def check(self):

        live = []
        usr_list = self.usr_list
        if not usr_list:
            logger.debug('无用户列表')
            return
        # url = 'https://api.twitch.tv/kraken/streams/sc2_ragnarok'

        stream = requests.get(API_ROOMS, headers=headers, params={'user_login': usr_list}, timeout=5)
        stream.close()

        data = stream.json()['data']
        if data:
            for i in data:
                live.append(self.use_id[i['user_id']])
        else:
            logger.debug('twitch无开播')

        return map(lambda x: self.usr_dict.get(x.lower()), live)


__plugin__ = Twitch
